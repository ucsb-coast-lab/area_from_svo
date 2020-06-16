#![feature(proc_macro_hygiene)]
use inline_python::{python,Context};

use std::f32::consts::PI;
use std::iter::{Iterator,FromIterator};
use ndarray::prelude::*;

fn main() {

    let r = 1.;


    let n1: f32 = 1.0; // Approximate index of refraction for air
    let n2: f32 = 1.49; // 1.49 Approximate index of refraction for Polymethyl methacrylate (acrylic)
    let n3: f32 = 1.33; // Approximate index of refration for fresh water

    let fov_v = 60f32.to_radians();
    let n_v = 1242.0;
    let fov_h = 90f32.to_radians();
    let n_h = 2208.0;

    let t3v = snell(n2,n3, snell(n1,n2,fov_v / 2.));
    let t3h = snell(n2,n3, snell(n1,n2,fov_h / 2.));

    let theta_v = t3v / PI;
    let theta_h = t3h / PI;

    let dH = (2.0 * PI * r) * theta_v / n_h;
    let dV = (2.0 * PI * r) * theta_h / n_v;
    let px_area = dV * dH;

    let c: f32 = ((4.0 * PI.powf(2.0) * theta_v * theta_h) / (n_v * n_h)).powf(2.0);

    let length = 25;
    let mut array = Array2::<f32>::zeros((length,4));
    for i in 0..length {
        
        let sigma_r = 0 as f32;
        let sigma_theta_h = i as f32;
        let sigma_theta_v = 0 as f32;
    
        let r_component = (2. * sigma_r / r).powf(2.0);
        let theta_v_component = (sigma_theta_v / theta_v).powf(2.0);
        let theta_h_component = (sigma_theta_h / theta_h).powf(2.0);
    
        let sigma_a_squared = c * (r_component + theta_h_component + theta_v_component );
        println!("sigma_a_squared: {}",sigma_a_squared);
        array[[i,0]] = r_component;
        array[[i,1]] = theta_h_component;
        array[[i,2]] = theta_v_component;
        array[[i,3]] = sigma_a_squared;
    }
    

    let x = to_vec(array.slice(s![.., 0,]).to_owned());
    let y = to_vec(array.slice(s![.., 1,]).to_owned());
    let z = to_vec(array.slice(s![.., 2,]).to_owned());
    let mut r = to_vec(array.slice(s![.., 3,]).to_owned());
    r = r.iter().map(|x| x * 1e11).collect::<Vec<f32>>();
  
    let ctx = Context::new();
    ctx.run(python! {
        import numpy as np;
        import matplotlib.pyplot as plt;
        from mpl_toolkits.mplot3d import Axes3D;

        fig = plt.figure()
        ax = fig.add_subplot(111, projection="3d")
        
        ax.scatter('x, 'y, 'z, s='r, c="blue", alpha=0.75)
        
        #plt.rc("text", usetex=True)
        #plt.rcParams["text.latex.preamble"]=[r"\usepackage{amsmath}"]
        ax.set_xlabel(r"radial contribution")
        ax.set_ylabel(r"theta_h contribution")
        ax.set_zlabel(r"theta_v contribution")
        
        plt.show()

      });

}

fn snell(n1: f32, n2:f32, theta: f32) -> f32 {
    ((n1 / n2) * theta.sin()).asin()
}

fn to_vec<T: std::clone::Clone>(arr: Array1<T>) -> Vec<T> {
    Array::from_iter(arr.iter().cloned()).to_vec()
}