use num::complex::Complex;

pub fn posPartOfSet() -> bool {}

fn compPartOfSet() -> bool {}

fn screenToComplex(w: u32, h: u32, x: u32, y: u32) -> Complex<f64> {}

fn calculateRatio(w: u32, h: u32) -> f64 {
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
    fn posPartOfSet_test() {}
}
