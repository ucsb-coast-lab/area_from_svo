#![feature(proc_macro_hygiene)]
use inline_python::{python, Context};

use ndarray::prelude::*;
use std::f64::consts::PI;
use std::iter::{FromIterator, Iterator};

fn main() {
    let r = 2000.;

    let n1: f64 = 1.0; // Approximate index of refraction for air
    let n2: f64 = 1.49; // 1.49 Approximate index of refraction for Polymethyl methacrylate (acrylic)
    let n3: f64 = 1.33; // Approximate index of refration for fresh water

    let fov_v = 60f64.to_radians();
    let n_v = 1242.0;
    let fov_h = 90f64.to_radians();
    let n_h = 2208.0;

    let t3v = snell(n2, n3, snell(n1, n2, fov_v / 2.));
    let t3h = snell(n2, n3, snell(n1, n2, fov_h / 2.));

    let theta_v = t3v / PI;
    let theta_h = t3h / PI;

    let dH = (2.0 * PI * r) * theta_v / n_h;
    let dV = (2.0 * PI * r) * theta_h / n_v;
    let px_area = dV * dH;

    // let c: f64 = ((4.0 * PI.powf(2.0) * theta_v * theta_h) / (n_v * n_h)).powf(2.0);
    let c: f64 = ((4.0 * PI.powf(2.0) * theta_v * theta_h) / (1.0 * 1.0)).powf(2.0);

    // let sigma_r = 100.;
    let sigma_r = 0.; // r/10.;
    let r_component = (0. * sigma_r / r).powf(2.0);
    let sigma_theta_v = 0f64.to_radians();
    let theta_v_component = (sigma_theta_v / theta_v).powf(2.0);
    let sigma_theta_h = 0f64.to_radians();
    let theta_h_component = (sigma_theta_h / theta_h).powf(2.0);
    let total_error = c * (r_component + theta_h_component + theta_v_component);
    let percent_r = c * (r_component) / total_error;
    let percent_theta_h = c * (theta_h_component) / total_error;
    let percent_theta_v = c * (theta_v_component) / total_error;
    
    println!("[s_r,s_th,s_tv]");
    println!(
        "[{},{},{}] -> total {} with relative percentages of [{},{},{}]",
        sigma_r,
        sigma_theta_h,
        sigma_theta_v,
        total_error,
        percent_r,
        percent_theta_h,
        percent_theta_v
    );

    let px_area_calc = std::f64::consts::PI * (r / 1000.).powf(2.0) / 10.0;
    println!("px_area of: {} mm^2  from simple calc",px_area_calc);
    println!("px_area of: {} mm^2 from dV*dH",px_area);
    println!("That means, as a percentage of the actual pixel area, the variation is: {:.2}%",(total_error/px_area) * 100.);
    let req_num_pixels = (163000./px_area) as u32;
    println!("That means for 163,000 mm^2, we'd need {} many pixels to cover it",req_num_pixels);
    let summed_object_error = req_num_pixels as f64 * total_error;
    println!("Multiplying the error for each pixel means we get a variation of: {} mm^2",summed_object_error);
    println!("which is {:.2}% of the 163,000 mm^2 true value",(summed_object_error/163000.)*100.);



    /* let ctx = Context::new();
    ctx.run(python! {
        import numpy as np;
        import matplotlib.pyplot as plt;
        from mpl_toolkits.mplot3d import Axes3D;

        fig = plt.figure()
        ax = fig.add_subplot(111, projection="3d")

        ax.scatter('x, 'y, 'z, s='r, c="blue", alpha=0.25)

        #plt.rc("text", usetex=True)
        #plt.rcParams["text.latex.preamble"]=[r"\usepackage{amsmath}"]
        ax.set_xlabel(r"radial contribution")
        ax.set_ylabel(r"theta_h contribution")
        ax.set_zlabel(r"theta_v contribution")

        plt.show()

      });*/
}

fn snell(n1: f64, n2: f64, theta: f64) -> f64 {
    ((n1 / n2) * theta.sin()).asin()
}

fn to_vec<T: std::clone::Clone>(arr: Array1<T>) -> Vec<T> {
    Array::from_iter(arr.iter().cloned()).to_vec()
}
