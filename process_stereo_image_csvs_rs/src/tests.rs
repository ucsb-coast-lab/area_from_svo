#[cfg(test)]
// Can run test with stdout output as `$ cargo test -- --nocapture`

mod tests {
    use crate::*;
    use lib::*;
    // extern crate test;  // This is an unstable import only available on Rust's nightly release
    use std::time::{Duration, Instant};

    #[test]
    fn it_works() {
        let now = Instant::now();
        let a = 4;
        assert_eq!(4,a);
        println!("{:?}",now.elapsed());
    }
}
