use core::f32;
use core::f32::consts::PI;

fn main() {

    println!("dH * dV = A\n");
    let r_list: Vec<f32> = vec![1000.0,1500.0,2000.0,2500.0,3000.0,3500.0,4000.0];
    for r in r_list {

        let n1: f32 = 1.0; // Approximate index of refraction for air
        let n2: f32 = 1.49; // 1.49 Approximate index of refraction for Polymethyl methacrylate (acrylic)
        let n3: f32 = 1.33; // Approximate index of refration for fresh water

        // Horizontal angle calculations
        let t1h: f32 = 45f32.to_radians();
        let t2h = ((n1 / n2) * t1h.sin()).asin();
        //println!("t2h = {}", t2h.to_degrees());
        let t3h = ((n2 / n3) * t2h.sin()).asin();
        //println!("t3 = {}", t3h.to_degrees());

        // Vertical angle calculations
        let t1v: f32 = 30f32.to_radians();
        let t2v = ((n1 / n2) * t1v.sin()).asin();
        //println!("t2h = {}", t2v.to_degrees());
        let t3v = ((n2 / n3) * t2v.sin()).asin();
        //println!("t3v = {}", t3v.to_degrees());

        let dH = (2.0 * PI * r) * (t3h.to_degrees() * 2.0 / 360.0) * (1.0 / 2208.0); // delta-Horizontal
        let dV = (2.0 * PI * r) * (t3v.to_degrees() * 2.0 / 360.0) * (1.0 / 1242.0);
        let px_area = dH * dV;
        println!("At r = {}: {} * {} = {}", r, dH, dV, px_area);
    }
}
