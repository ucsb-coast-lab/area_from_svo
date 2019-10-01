#![allow(dead_code)]
#![allow(unused_parens)]

// FFI dependencies
use std::ffi::CStr;
use std::os::raw::c_char;

use std::f32;
use std::fs::File;

use image::{ImageBuffer, RgbImage};

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
    // println!("About to attempt parsing the filename_arg in Rust");
    let csv_filename = parse_filename_from_c(filename_arg);
    // println!("Parsed csv_filname of: {} from C++ argument", csv_filename);
    print_area(&csv_filename);
}

pub fn print_area(filename: &str) {
    let csv_path = filename.split("stereo_image_csvs/").collect::<Vec<&str>>();
    println!("csv_path: {}", csv_path[1]);
    let parsed_path = csv_path[1].split(".csv").collect::<Vec<&str>>();
    // println!("image_path except csv: {}", parsed_path[0]);
    let image_path = "processed_images/".to_owned() + &parsed_path[0].to_owned() + ".png";
    println!("The processed image will be saved to: {}", image_path);
    let arr: Vec<StereoPixel> = convert_records_to_array(&filename);
    let target_pixels: f32 =
        convert_array_to_image_and_get_number_of_target_pixels(arr, &image_path);
}

fn convert_records_to_array(file_path: &str) -> Vec<StereoPixel> {
    let file = File::open(file_path).expect("Couldn't open the file"); //?;
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

fn convert_array_to_image_and_get_number_of_target_pixels(
    arr: Vec<StereoPixel>,
    image_path: &str,
) -> f32 {
    let mut img: RgbImage = ImageBuffer::new(2208, 1242);
    let (w, h) = img.dimensions();
    let mut target_pixels: f32 = 0.0;
    let mut sum_area: f32 = 0.0;

    for x in 0..w {
        for y in 0..h {
            let col = x as usize;
            let row = y as usize;
            let sp = arr[(WIDTH * row) + col];
            let b = sp.b;
            let g = sp.g;
            let r = sp.r;
            let z = sp.z;

            // Pixel needs to go in b,g,r order
            let mut pixel = image::Rgb([b, g, r]);
            let col = x as usize;
            let row = y as usize;

            // Do some filtering here
            // Something's off here; I think the R,B values might be flipped between what I think
            // they are and which actually calls which one. However, since the problem seems to
            // be going back to where we're actually getting things from the .svo file in the
            // C++ program, I'm going to ignore that for now.

            //let pixel_sum = (r.pow(2) as f32 + g.pow(2) as f32 + b.pow(2) as f32).powf(0.5);
            //println!("pixel_sum = {} at z = {}",pixel_sum,z);

            // TO_DO: These filters should be written as functions, where the pixel can get passed
            // to them

            // The filter r < 70 (I think this actually corresponds to b < 70) && z < 2000
            // should isolate the slate target in the pool .svo file
            if image_path.contains("pool") {
                if arr[(WIDTH * row) + col].r < 70
                    && arr[(WIDTH * row) + col].g < 45 // Having trouble balancing this parameter between 3_0 and 1_5 pool images
                    && arr[(WIDTH * row) + col].r > 35
                    && arr[(WIDTH * row) + col].z < 3000.0
                    && row > 660
                    && arr[(WIDTH * row) + col].z != 0.0
                {
                    //dbg!(pixel_sum);
                    pixel[0] = b; // b
                    pixel[1] = g; // g
                    pixel[2] = r; // r
                    target_pixels = target_pixels + 1.0;
                    let corrected_distance: f32 = (arr[(WIDTH * row) + col].z / DCF) / 1000.0;
                    let px_area = (12500.0 / (125918.97 * (-1.1724 * corrected_distance).exp()));
                    //println!("px_area of target pixel {} @ corrected z = {}: {}",target_pixels,corrected_distance,px_area);
                    sum_area += px_area;
                } else {
                    pixel[0] = 0;
                    pixel[1] = 0;
                    pixel[2] = 0;
                }
            }
            // Filter for the lab slate as a control (size ~ 126 mm x 100 mm)
            else if image_path.contains("table") {
                if (b as i8 - 56).abs() < 10
                    && (g as i8 - 56).abs() < 10
                    && (r as i8 - 70).abs() < 10
                    && z < 3000.0
                    && sp.pixel_x > 1100
                {
                    //dbg!(pixel_sum);
                    pixel[0] = b; // b
                    pixel[1] = g; // g
                    pixel[2] = r; // r
                    target_pixels = target_pixels + 1.0;
                } else {
                    pixel[0] = 0;
                    pixel[1] = 0;
                    pixel[2] = 0;
                }
            } else if image_path.contains("tank") {
                if arr[(WIDTH * row) + col].r < 80
                    && arr[(WIDTH * row) + col].g < 66
                    && arr[(WIDTH * row) + col].z < 2500.0
                    && row > 870
                {
                    //dbg!(pixel_sum);
                    pixel[0] = b; // b
                    pixel[1] = g; // g
                    pixel[2] = r; // r
                    target_pixels = target_pixels + 1.0;
                    let corrected_distance: f32 = (arr[(WIDTH * row) + col].z / DCF) / 1000.0;
                    let px_area = (12500.0 / (125918.97 * (-1.1724 * corrected_distance).exp()));
                    //println!("px_area of target pixel {} @ corrected z = {}: {}",target_pixels,corrected_distance,px_area);
                    sum_area += px_area;
                } else {
                    pixel[0] = 0;
                    pixel[1] = 0;
                    pixel[2] = 0;
                }
            }
            //let pixel = image::Rgb([b, g, r])
            img.put_pixel(x, y, pixel);
        }
    }
    println!("The summed px_area total of {} based on {} slate-assigned pixels is {} mm^2, or {:.2}% of the 12500 ideal",image_path,target_pixels,sum_area,sum_area/12500.0);
    img.save(image_path).unwrap();
    // Returns the total number of pixels assigned to the targete slate in this image as a u32 value
    target_pixels
}
