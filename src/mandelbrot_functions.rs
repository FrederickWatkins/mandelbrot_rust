use num::Complex;

pub fn pos_part_of_set(w:u32, h:u32, x:u32, y:u32, iterations: u32) -> bool {
    comp_part_of_set(screen_to_complex(w, h, x, y), iterations)
}

fn comp_part_of_set(c: Complex<f64>, iterations: u32) -> bool {
    let mut z = Complex::<f64>::new(0.0, 0.0);
    for i in 0..iterations {
        z = (z * z) + c;
        if z.norm() > 6000000.0 {
            return false;
        }
    }
    z.norm() < 1000.0
}

fn screen_to_complex(w: u32, h: u32, x: u32, y: u32) -> Complex<f64> {
    let wf: f64 = w.into();
    let hf: f64 = h.into();
    let xf: f64 = x.into();
    let yf: f64 = y.into();

    let ratio = calculate_ratio(wf, hf);

    let r = (xf - (wf / 1.5)) * ratio;
    let i = ((hf / 2.0) - yf) * ratio;
    Complex::<f64>::new(r, i)
}

fn calculate_ratio(w: f64, h: f64) -> f64 {
    if w > h * 1.5 {
        2.0 / h
    } else {
        3.0 / w
    }
}

#[cfg(test)]
mod mandelbrot_functions_test {
    use super::*;

    #[test]
    fn calculate_ratio_test_1() {
        assert_eq!(calculate_ratio(500.0, 500.0), 3.0/500.0);
    }

    #[test]
    fn calculate_ratio_test_2() {
        assert_eq!(calculate_ratio(2000.0, 500.0), 2.0/500.0);
    }
}