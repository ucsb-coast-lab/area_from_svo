use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::process;

use image::{ImageBuffer, RgbImage};

#[derive(Copy, Clone, Debug)]
struct StereoPixel {pixel_x: u32, pixel_y: u32, x: f32, y:f32, z:f32, r:u8, g:u8, b:u8}

impl StereoPixel {
    fn default() -> Self {
        StereoPixel  {pixel_x: 0, pixel_y: 0, x: 0.0, y: 0.0, z: 0.0, r: 0, g:0, b:0}
    }
}

type Record = (u32, u32, f32, f32, f32, u8, u8, u8);

fn main() {
    let mut arr = [[StereoPixel::default(); 2208]; 1242];
    if let Err(err) = convert_record_to_array(arr) {
        println!("{}", err);
        process::exit(1);
    }
}

/// Returns the first positional argument sent to this process. If there are no
/// positional arguments, then this returns an error.
fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn convert_record_to_array(mut arr: [[StereoPixel; 2208]; 1242]) -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let file = File::open(file_path)?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .flexible(true)
        .from_reader(file);

    for result in rdr.deserialize() {
        let record: Record = result?;
        let pixel_x = record.0;
        let pixel_y = record.1;
        // Assigns all StereoPixel information to the array, to be available
        arr[pixel_x as usize][pixel_y as usize].pixel_x = pixel_x;
        arr[pixel_x as usize][pixel_y as usize].pixel_y = pixel_y;
        arr[pixel_x as usize][pixel_y as usize].x = record.2;
        arr[pixel_x as usize][pixel_y as usize].y = record.3;
        arr[pixel_x as usize][pixel_y as usize].z = record.4;
        arr[pixel_x as usize][pixel_y as usize].r = record.5;
        arr[pixel_x as usize][pixel_y as usize].g = record.6;
        arr[pixel_x as usize][pixel_y as usize].b = record.7;
    }

    Ok(())
}

fn array_to_rgb_image(arr: [[StereoPixel; 2208]; 1242]) {
    let mut img: RgbImage = ImageBuffer::new(2208, 1242);
    let (h,w) = img.dimensions();
    for x in 0..w {
        for y in 0..h {
            // Pixel needs to go in b,g,r order
            let mut pixel = image::Rgb([0,0,0]);

            // Do some filtering here
            // Something's off here; I think the R,B values might be flipped between what I think
            // they are and which actually calls which one. However, since the problem seems to
            // be going back to where we're actually getting things from the .svo file in the
            // C++ program, I'm going to ignore that for now.

            //let pixel_sum = (r.pow(2) as f32 + g.pow(2) as f32 + b.pow(2) as f32).powf(0.5);
            //println!("pixel_sum = {} at z = {}",pixel_sum,z);

            // The filter r < 70 (I think this actually corresponds to b < 70) && z < 2000
            // should isolate the slate target in the pool .svo file
            if arr[x as usize][y as usize].r < 70 && arr[x as usize][y as usize].z < 2000.0 {
                //dbg!(pixel_sum);
                pixel[0] = arr[x as usize][y as usize].b; // b
                pixel[1] = arr[x as usize][y as usize].g; // g
                pixel[2] = arr[x as usize][y as usize].r; // r
            }
            else {
                pixel[0] = 0;
                pixel[1] = 0;
                pixel[2] = 0;
            }


            //let pixel = image::Rgb([b, g, r])
            img.put_pixel(x,y,pixel);

        }
    }
    img.save("test.png").unwrap();
}
