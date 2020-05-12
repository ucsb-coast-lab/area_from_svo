#![allow(dead_code)]
#![allow(unused_parens)]
#![allow(unused_imports)]

// FFI dependencies
use std::ffi::CStr;
use std::os::raw::c_char;

use std::f32;
use std::fs::File;

use image::load_from_memory;
use image::{GenericImage, GenericImageView, ImageBuffer, Luma, Pixel, RgbImage, Rgba, RgbaImage};
//use image::buffer::Pixel;
use imageproc::contrast::{stretch_contrast, threshold_mut};

// Each line of the calibration csv files is ultimately deserialized into this struct
#[derive(Copy, Clone, Debug)]
struct StereoPixel {
    pixel_x: u32,
    pixel_y: u32,
    x: f32,
    y: f32,
    z: f32,
    r: u8,
    g: u8,
    b: u8,
}

impl StereoPixel {
    fn default() -> Self {
        StereoPixel {
            pixel_x: 0,
            pixel_y: 0,
            x: 0.0,
            y: 0.0,
            z: 0.0,
            r: 0,
            g: 0,
            b: 0,
        }
    }
}

#[derive(Clone, Debug)]
struct CalibrationSet {
    filename: String,
    target_pixels: f32,
    sqmmpp: f32, // target pixels per square millimeter
}

const WIDTH: usize = 2208; // Width of the .csv image in pixels
const HEIGHT: usize = 1242; // Height of the .csv image in pixels
const TARGET_AREA: f32 = 12500.0; // Area of the slate target in mm^2
const DCF: f32 = 0.746; // Distance correction factor, calculated from Excel

// Used for deserializing the csv file results, then converts to StereoPixel struct
type Record = (u32, u32, f32, f32, f32, u8, u8, u8);

#[no_mangle]
pub extern "C" fn hello_from_rust_rs() {
    println!("Hello from Rust!");
}

fn parse_filename_from_c(filename_arg: *const c_char) -> String {
    // println!("About to check if filename_arg is null...");
    assert!(!filename_arg.is_null());
    // println!("Parsed filenname is not null!");
    let filename_c_str = unsafe { CStr::from_ptr(filename_arg) };
    let filename = filename_c_str.to_str().expect("Not a valid UTF-8 string");
    filename.to_string()
}

#[no_mangle]
pub extern "C" fn open_file_rs(filename_arg: *const c_char) {
    let filename = parse_filename_from_c(filename_arg);
    // println!("filename_arg: {}",filename);
    open_file(&filename);
}

fn open_file(filename: &str) {
    let _file = File::open(filename).expect("Specified file does not exist");
    // println!("The file at {} was opened!", filename);
}

#[no_mangle]
pub extern "C" fn print_area_rs(filename_arg: *const c_char) {
    //println!("About to attempt parsing the filename_arg in Rust");
    let csv_filename = parse_filename_from_c(filename_arg);
    // println!("Parsed csv_filname of: {} from C++ argument", csv_filename);
    print_area(&csv_filename);
}

pub fn print_area(filename: &str) {
    let csv_path = filename.split("stereo_image_csvs/").collect::<Vec<&str>>();
    //println!("csv_path: {}", csv_path[1]);
    let parsed_path = csv_path[1].split(".csv").collect::<Vec<&str>>();
    //println!("image_path except csv: {}", parsed_path[0]);
    let image_path = "processed_images/".to_owned() + &parsed_path[0].to_owned() + ".png";
    //println!("The processed image will be saved to: {}", image_path);
    let frame_num = &parsed_path[0].split("_").collect::<Vec<&str>>();
    print!("{},", frame_num[frame_num.len() - 1]);

    let arr: Vec<StereoPixel> = convert_records_to_array(&filename);
    let target_pixels: f32 =
        convert_array_to_image_and_get_number_of_target_pixels(arr, &image_path);
    // For debugging use the below println! line
    // println!("For {}, based on an ideal area of 12500 mm^2 and {} target-assigned pixels, the per-pixel area is {}\n",filename, target_pixels,12500f32/(target_pixels as f32));
}

fn convert_records_to_array(file_path: &str) -> Vec<StereoPixel> {
    // println!("Attempting to open the stereo image csv at path: {}",file_path);
    let file = File::open(file_path).expect("Couldn't open the stereo image .csv file"); //?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .flexible(true)
        .from_reader(file);

    let mut arr: Vec<StereoPixel> = Vec::with_capacity(WIDTH * HEIGHT);
    for _ in 0..((WIDTH * HEIGHT) + 1) {
        arr.push(StereoPixel::default());
    }

    let mut i = 0;
    for result in rdr.deserialize() {
        let record: Record = result.unwrap(); //?;
                                              // println!("record[{}] = {:?}",i,record);
        let pixel_x = record.0;
        let pixel_y = record.1;

        // Indexing the array by (WIDTH * row) + col
        let col = pixel_x as usize;
        let row = pixel_y as usize;

        // Assigns all StereoPixel information to the array, to be available
        arr[(WIDTH * row) + col].pixel_x = pixel_x;
        arr[(WIDTH * row) + col].pixel_y = pixel_y;
        arr[(WIDTH * row) + col].x = record.2;
        arr[(WIDTH * row) + col].y = record.3;
        arr[(WIDTH * row) + col].z = record.4;
        arr[(WIDTH * row) + col].r = record.5;
        arr[(WIDTH * row) + col].g = record.6;
        arr[(WIDTH * row) + col].b = record.7;

        i = i + 1;
    }

    arr
}

use image::imageops::grayscale;

fn convert_array_to_image_and_get_number_of_target_pixels(
    arr: Vec<StereoPixel>,
    image_path: &str,
) -> f32 {
    //let img: RgbImage = ImageBuffer::new(2208, 1242);
    let (w, h) = (2208, 1242);
    let mut target_pixels: f32 = 0.0;
    let mut sum_area: f32 = 0.0;

    //let mut my_image: RgbaImage = ImageBuffer::new(w, h);
    let mut naive = image::DynamicImage::new_rgba8(w, h);

    // Build the initial GenericImage structure
    for y in 0..h {
        for x in 0..w {
            let sp = arr[(WIDTH * (y as usize)) + (x as usize)];
            naive.put_pixel(x, y, image::Rgba([sp.b, sp.g, sp.r, 255]))
        }
    }
    // Run the preliminary image processing over it
    let mut luma_image = naive.to_luma();
    let stretched = stretch_contrast(&mut luma_image, 33, 45);

    let mut final_canvas = image::ImageBuffer::new(w, h);

    for y in 0..h {
        for x in 0..w {
            let sp = arr[(WIDTH * (y as usize)) + (x as usize)];
            let radial_dist = (sp.x.powf(2.0) + sp.y.powf(2.0) + sp.z.powf(2.0)).powf(0.5);
            let corrected_distance: f32 = (sp.z / DCF) / 1000.0; // Note: This is the one that has been used for most of the preliminary analysis
            //let corrected_distance: f32 = (radial_dist / DCF) / 1000.0;
            let mut pulled_pixel = stretched.get_pixel(x, y).to_luma(); // Comment to to_luma() for color

            // Lines for debugging
            //println!("{:?}",pulled_pixel);
            //println!("corrected_distance: {:?}",corrected_distance);
            if corrected_distance < 4.0
                && corrected_distance > 0.0
                && sp.b < 100
                && sp.g < 80
            {
                //final_canvas.put_pixel(x,y,image::Rgba([sp.b, sp.g, sp.r,255]));
                pulled_pixel[0] = 255; // For luma image result

                target_pixels = target_pixels + 1.0;
                let px_area = std::f32::consts::PI * (corrected_distance).powf(2.0) / 10.0;
                sum_area += px_area;
            } else {
                //final_canvas.put_pixel(x,y,image::Rgba([0,0,0,255]));
                pulled_pixel[0] = 0;
            }
            final_canvas.put_pixel(x, y, pulled_pixel);
        }
    }

    // For de-bugging the following println! should be used
    //println!("Based on the summed area of distance-corrected target-assigned pixels ({}), the slate area is: {} with an average area of {} mm^2/pixel",target_pixels,sum_area,sum_area/target_pixels);
    println!("{}", sum_area);
    // img.save(image_path).expect("Was not able to save modified image to file");

    //stretched.save(image_path).expect("Was not able to save modified image to file");
    final_canvas
        .save(image_path)
        .expect("Was not able to save modified image to file");

    // Returns the total number of pixels assigned to the targete slate in this image as a u32 value
    target_pixels
}
