use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::process;

use image::{ImageBuffer, RgbImage};

type Record = (u32, u32, f32, f32, f32, u8, u8, u8);

fn run() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let file = File::open(file_path)?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .flexible(true)
        .from_reader(file);

    let mut img: RgbImage = ImageBuffer::new(2208, 1242);

    for result in rdr.deserialize() {
        let record: Record = result?;
        let pixel_x = record.0;
        let pixel_y = record.1;
        let _x = record.2;
        let _y = record.3;
        let z = record.4;
        let r = record.5;
        let g = record.6;
        let b = record.7;

        // Pixel needs to go in b,g,r order
        let mut pixel = image::Rgb([b,g,r]);

        // Do some filtering here
        /*
        let pixel_sum = (r.pow(2) as f32 + g.pow(2) as f32 + b.pow(2) as f32).powf(0.5);
        //println!("pixel_sum = {} at z = {}",pixel_sum,z);
        if pixel_sum > 23.0 && z > 0.01 && z < 2000.0 {
            //dbg!(pixel_sum);
            pixel[0] = b;
            pixel[1] = g;
            pixel[2] = r;
        }
        else {
            pixel[0] = 0;
            pixel[1] = 0;
            pixel[2] = 0;
        }
        */

        //let pixel = image::Rgb([b, g, r])
        img.put_pixel(pixel_x, pixel_y, pixel);

    }
    img.save("test.png").unwrap();
    Ok(())
}

/// Returns the first positional argument sent to this process. If there are no
/// positional arguments, then this returns an error.
fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
