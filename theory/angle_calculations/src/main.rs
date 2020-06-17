#![feature(proc_macro_hygiene)]
use inline_python::{python,Context};

use core::f32;
use core::f32::consts::PI;

fn main() {

    println!("dH * dV = A\n");
    let c = Context::new();
    c.run(python! {
        import numpy as np;
        import matplotlib.pyplot as plt;
        fig = plt.figure();
      });
    let r_list: Vec<f32> = vec![1000.0,1500.0,2000.0,2500.0,3000.0,3500.0,4000.0];
    let mut px_areas: Vec<f32> = Vec::with_capacity(r_list.len());
    for r in &r_list {

        let n1: f32 = 1.0; // Approximate index of refraction for air
        let n2: f32 = 1.49; // 1.49 Approximate index of refraction for Polymethyl methacrylate (acrylic)
        let n3: f32 = 1.33; // Approximate index of refration for fresh water

        let fov_h = 90f32.to_radians(); // ZED Camera horizontal field of view
        let n_h = 2208.0;  // Number of pixels along the horizontal axis
        let fov_v = 60f32.to_radians(); // ZED Camera vertical field of view
        let n_v = 1242.0;    // Number of pixels along the vertical axis

        // Horizontal angle calculations
        let t1h: f32 = fov_h / 2.; // Radians is used because of the std::f32 args
        // let t2h = snell(n1,n2,t1h);
        // let t3h = snell(n2,n3,t2h);
        let t3h = snell(n2,n3, snell(n1,n2,fov_h / 2.));
        // Vertical angle calculations
        let t1v: f32 = fov_v / 2.;
        // let t2v = snell(n1,n2,t1v);
        // let t3v = snell(n2,n3,t2v);
        let t3v = snell(n2,n3, snell(n1,n2,fov_v / 2.));
        //println!("t3v = {}", t3v.to_degrees());

        let theta_v = t3v / PI; // Used in the theory calcs
        let theta_h = t3h / PI; // Used in the theory calcs

        let dH = (2.0 * PI * r) * theta_v / n_h;
        let dV = (2.0 * PI * r) * theta_h / n_v;
        px_areas.push(dH * dV);
        println!("At r = {}: {} * {} = {}", r, dH, dV, dV*dH);
    }
    c.run(python! {
        z = np.poly1d(np.polyfit('r_list,'px_areas, 2));
        print(z)
        plt.plot('r_list,'px_areas,"bo");
        plt.plot('r_list, z('r_list), "b-");
        plt.text(0.1,
            0.8,
            "z = {}".format(z),
            transform=plt.gca().transAxes);
        plt.show();
    });
}

// Calculates resulting angle from a material with a different index of refraction
fn snell(n1: f32, n2:f32, theta: f32) -> f32 {
    ((n1 / n2) * theta.sin()).asin()
}