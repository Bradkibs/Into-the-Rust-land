use std::fs::File;
use std::io;
use num::complex::Complex;
use std::io::prelude::*;


fn calculate_mandelbrot(

                              max_iters: usize,
                              x_min: f64,
                              x_max: f64,
                              y_min: f64,
                              y_max: f64,
                              width: usize,
                              height: usize,
) -> Vec<Vec<usize>> {

    let mut rows: Vec<_> = Vec::with_capacity(width);
    for img_y in 0..height {                          // <7>

        let mut row: Vec<usize> = Vec::with_capacity(height);
        for img_x in 0..width {

            let x_percent = img_x as f64 / width as f64;
            let y_percent = img_y as f64 / height as f64;
            let cx = x_min + (x_max - x_min) * x_percent;
            let cy = y_min + (y_max - y_min) * y_percent;
            let escaped_at = mandelbrot_at_point(cx, cy, max_iters);
            row.push(escaped_at);
        }

        rows.push(row);
    }
    rows
}

fn mandelbrot_at_point(
                          cx: f64,
                          cy: f64,
                          max_iters: usize,
) -> usize {
    let mut z = Complex { re: 0.0, im: 0.0 };
    let c = Complex::new(cx, cy);

    for i in 0..=max_iters {
        if z.norm() > 2.0 {
            return i;
        }
        z = z * z + c;
    }
    max_iters
}

fn render_mandelbrot(escape_vals: Vec<Vec<usize>>) {
    let mut file = File::create("mandelbrot.txt").expect("unable to create the file!");
    for row in escape_vals {
        let mut line = String::with_capacity(row.len());
        for column in row {
            let val = match column {
                0..=2 => ' ',
                3..=5 => '.',
                6..=10 => '•',
                11..=30 => '*',
                31..=100 => '+',
                101..=200 => 'x',
                201..=400 => '$',
                401..=700 => '#',
                _ => '%',
            };

            line.push(val);
        }
        file.write_all(line.as_bytes()).expect("Unable to write the to mandelbrot.txt");
        file.write_all(b"\n").expect("Unable to write a newline into the txt file");
        println!("{}", line);
    }
}

fn main() {
    println!("Please enter the height of the mandelbrot set you wish to display");
    let mut res_h = String::new();
    let mut res_w = String::new();
    io::stdin().read_line(&mut res_h).expect("Error please use a valid number");
    println!("Please enter the width of the mandelbrot set you wish to display");
    io::stdin().read_line(&mut res_w).expect("Error please use a valid number");
    let height = res_h.trim().parse::<usize>().unwrap();
    let width = res_w.trim().parse::<usize>().unwrap();
    let mandelbrot = calculate_mandelbrot(1000, -2.0, 1.0, -1.0,
                                          1.0, width, height);

    render_mandelbrot(mandelbrot);
}