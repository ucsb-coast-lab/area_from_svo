mod clustering;
mod lib;
mod tests;

use clustering::print_area_from_clusters;
use lib::print_area;

use std::env;
use std::error::Error;
use std::ffi::OsString;

// Can be used to check area calculations on a pre-built stereo image .csv file
// This will always panic at the moment, since the hard-coded paths for the modified
// image folder doesn't exist in this directory
fn main() {
    let file_path = match get_first_arg() {
        Ok(path) => path,
        Err(error) => panic!(
            "Error: Provided file_path string was not considered valid because {}",
            error
        ),
    };

    let path = match file_path.into_string() {
        Ok(path) => path,
        Err(_error) => panic!("Couldn't convert OsString to String"),
    };
    // print_area(&path);
    print_area_from_clusters(&path,3,0.0);

}

/// Returns the first positional argument sent to this process. If there are no
/// positional arguments, then this returns an error.
fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}
