#![feature(proc_macro_hygiene)]
use inline_python::{python, Context};

use ndarray::prelude::*;
use std::f64::consts::PI;
use std::iter::{FromIterator, Iterator};

fn main() {
    let r = 2000.;

    let sigma_r = 0.;
    let sigma_theta_h = 0f64.to_radians(); // in degrees
    let sigma_theta_v = 0f64.to_radians(); // in degrees

    let mut sigma_r_values: Vec<f64> = Vec::new();
    let mut sigma_r_errors: Vec<f64> = Vec::new();

    let mut sigma_theta_v_values: Vec<f64> = Vec::new();
    let mut sigma_theta_v_errors: Vec<f64> = Vec::new();

    let mut sigma_theta_h_values: Vec<f64> = Vec::new();
    let mut sigma_theta_h_errors: Vec<f64> = Vec::new();


    for i in 0..10 {
        let sigma_r = (i as f64 / 100. ) * r; 
        sigma_r_values.push(sigma_r);
        let error = calculate_errors(r,sigma_r,sigma_theta_v,sigma_theta_h);
        sigma_r_errors.push(error);
    }

    for i in 0..10 {
        let sigma_theta_v = (i as f64 / 5. ).to_radians(); 
        sigma_theta_v_values.push(sigma_theta_v);
        let error = calculate_errors(r,sigma_r,sigma_theta_v,sigma_theta_h);
        sigma_theta_v_errors.push(error);
    }

    for i in 0..10 {
        let sigma_theta_h = (i as f64 / 5. ).to_radians(); 
        sigma_theta_h_values.push(sigma_theta_h);
        let error = calculate_errors(r,sigma_r,sigma_theta_v,sigma_theta_h);
        sigma_theta_h_errors.push(error);
    }

    let r = r as u32; // For use in the plots
    
    let ctx = Context::new();
    ctx.run(python! {
        import numpy as np;
        import matplotlib.pyplot as plt;
        
        fig, ax1 = plt.subplots()
        ax2 = ax1.twinx()
        plot_v = ax1.plot('sigma_theta_v_errors,'sigma_theta_v_values,"r-",label="$\\sigma_\\theta$$_V$");
        plot_h = ax1.plot('sigma_theta_h_errors,'sigma_theta_h_values,"g-",label="$\\sigma_\\theta$$_H$");
        plot_r = ax2.plot('sigma_r_errors,'sigma_r_values,"b-",label="$\\sigma_r$");
        
        lns = plot_v + plot_h + plot_r;
        labs = [l.get_label() for l in lns]
        ax1.legend(lns, labs, loc="lower right")

        ax1.set_xlabel("Uncertainty in Pixel Area [%]")
        ax1.set_ylabel("$\\sigma_\\theta$ [radians] ")
        ax2.set_ylabel("$\\sigma_r$ [mm]")

        //ax1.legend(loc="lower right")
        //ax2.legend(loc=0)
        plt.title("Error Contribution to Pixel Area For $\\theta_v$ at r = {} mm".format('r))
        
        plt.show()

      });
}

fn calculate_errors(r: f64, sigma_r: f64, sigma_theta_v: f64, sigma_theta_h: f64) -> f64 {
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
    let c: f64 = (    (4.0 * PI.powf(2.0) * theta_v * theta_h) / (1.0 * 1.0)   ).powf(2.0);

    // let sigma_r = 100.; // mm
    // let sigma_r = 100.; // r/10.;
    let r_component = (2. * sigma_r / r).powf(2.0);
    // let sigma_theta_v = 0f64.to_radians();
    let theta_v_component = (sigma_theta_v / theta_v).powf(2.0);
    // let sigma_theta_h = 0f64.to_radians();
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
      
    
    
    let percentage_of_pixel = (total_error/px_area) * 100.;
    println!("That means, as a percentage of the actual pixel area, the variation is: {:.2}%",percentage_of_pixel);
    let req_num_pixels = (163000./px_area) as u32;
    println!("That means for 163,000 mm^2, we'd need {} many pixels to cover it",req_num_pixels);
    let summed_object_error = req_num_pixels as f64 * total_error;
    println!("Multiplying the error for each pixel means we get a variation of: {} mm^2",summed_object_error);
    println!("which is {:.2}% of the 163,000 mm^2 true value",(summed_object_error/163000.)*100.);
    percentage_of_pixel
}

fn snell(n1: f64, n2: f64, theta: f64) -> f64 {
    ((n1 / n2) * theta.sin()).asin()
}

fn to_vec<T: std::clone::Clone>(arr: Array1<T>) -> Vec<T> {
    Array::from_iter(arr.iter().cloned()).to_vec()
}
