use num::Complex;

pub fn pos_part_of_set(w:u32, h:u32, x:u32, y:u32, iterations: u32) -> bool {
    comp_part_of_set(screen_to_complex(w, h, x, y), iterations)
}

fn comp_part_of_set(c: Complex<f64>, iterations: u32) -> bool {
    let z = Complex::<f64>::new(0.0, 0.0);
    for i in 0..iterations {
        z = (z * z) + c;
        if z.norm() > 6000000 {
            false
        }
    }
}

fn screen_to_complex(w: u32, h: u32, x: u32, y: u32) -> Complex<f64> {
    let ratio = calculate_ratio(w, h);
    let r = (x.into() - (w.into() / 1.5)) * ratio;
    let i = ((h.into() / 2) - y.into()) * ratio;
    Complex::<f64>::new(r, i)
}

fn calculate_ratio(w: u32, h: u32) -> f64 {
    let ratio: f64;
    let wf: f64 = w.into();
    let hf: f64 = h.into();
    if wf > hf * 1.5 {
        ratio = 2.0 / hf;
    } else {
        ratio = 3.0 / wf;
    }
    ratio
}

#[cfg(test)]
mod mandelbrot_functions_test {
    #[test]
    fn pos_part_of_set_test1() {
    }
}
