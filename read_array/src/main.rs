use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::process;
use std::{thread, time};
use std::vec;

use image::{ImageBuffer, RgbImage};
use ndarray::Array2;

#[derive(Copy, Clone, Debug)]
struct StereoPixel {pixel_x: u32, pixel_y: u32, x: f32, y:f32, z:f32, r:u8, g:u8, b:u8}

impl StereoPixel {
    fn default() -> Self {
        StereoPixel  {pixel_x: 0, pixel_y: 0, x: 0.0, y: 0.0, z: 0.0, r: 0, g:0, b:0}
    }
}

const WIDTH: usize = 2208;
const HEIGHT: usize = 1242;

type Record = (u32, u32, f32, f32, f32, u8, u8, u8);

fn main() {
    let arr: Vec<StereoPixel> = convert_record_to_array();
    array_to_rgb_image(arr);
}

fn convert_record_to_array() -> Vec<StereoPixel> {

    let file_path = get_first_arg().expect("Error accepting command line arg");//?;
    let file = File::open(file_path).expect("Couldn't open the file");//?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .flexible(true)
        .from_reader(file);

    let mut arr: Vec<StereoPixel> = Vec::with_capacity(WIDTH*HEIGHT);
    for _ in 0..((WIDTH*HEIGHT) + 1) {
        arr.push(StereoPixel::default());
    }

    let mut i = 0;
    for result in rdr.deserialize() {
        let record: Record = result.unwrap();//?;
        //println!("record[{}] = {:?}",i,record);
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

fn array_to_rgb_image(arr: Vec<StereoPixel>) {
    let mut img: RgbImage = ImageBuffer::new(2208, 1242);
    let (w,h) = img.dimensions();

    for x in 0..w {
        for y in 0..h {
            // Pixel needs to go in b,g,r order
            let mut pixel = image::Rgb([0,0,0]);
            let col = x as usize;
            let row = y as usize;

            // Do some filtering here
            // Something's off here; I think the R,B values might be flipped between what I think
            // they are and which actually calls which one. However, since the problem seems to
            // be going back to where we're actually getting things from the .svo file in the
            // C++ program, I'm going to ignore that for now.

            //let pixel_sum = (r.pow(2) as f32 + g.pow(2) as f32 + b.pow(2) as f32).powf(0.5);
            //println!("pixel_sum = {} at z = {}",pixel_sum,z);

            // The filter r < 70 (I think this actually corresponds to b < 70) && z < 2000
            // should isolate the slate target in the pool .svo file
            if arr[(WIDTH * row) + col].r < 70 && arr[(WIDTH * row) + col].z < 2000.0 {
                //dbg!(pixel_sum);
                pixel[0] = arr[(WIDTH * row) + col].b; // b
                pixel[1] = arr[(WIDTH * row) + col].g; // g
                pixel[2] = arr[(WIDTH * row) + col].r; // r
            }
            else {
                pixel[0] = 30;
                pixel[1] = 0;
                pixel[2] = 0;
            }


            //let pixel = image::Rgb([b, g, r])
            img.put_pixel(x,y,pixel);

        }
    }
    img.save("test.png").unwrap();
}

/// Returns the first positional argument sent to this process. If there are no
/// positional arguments, then this returns an error.
fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but gzot none")),
        Some(file_path) => Ok(file_path),
    }
}
