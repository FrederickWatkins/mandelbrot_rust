use num::Complex;
#[derive(Copy, Clone)]
struct MandelbrotCalculator {
    width: f64,
    height: f64,
    ratio: f64,
    iterations: u32,
}

impl MandelbrotCalculator {
    pub fn new(width: u32, height: u32, iterations: u32) -> Self {
        let mut new = MandelbrotCalculator {
            width: 0.0,
            height: 0.0,
            ratio: 0.0,
            iterations: 0,
        };
        new.change_config(width, height, iterations);
        new
    }

    pub fn pos_part_of_set(self, x: u32, y: u32) -> bool {
        self.comp_part_of_set(self.screen_to_complex(x, y))
    }

    pub fn comp_part_of_set(self, c: Complex<f64>) -> bool {
        let mut z = Complex::<f64>::new(0.0, 0.0);
        for _i in 0..self.iterations {
            z = (z * z) + c;
            if z.norm() > 6000000.0 {
                return false;
            }
        }
        z.norm() < 1000.0
    }

    fn screen_to_complex(self, x: u32, y: u32) -> Complex<f64> {
        let xf: f64 = x.into();
        let yf: f64 = y.into();

        let r = (xf - (self.width / 1.5)) * self.ratio;
        let i = ((self.height / 2.0) - yf) * self.ratio;
        Complex::<f64>::new(r, i)
    }

    fn change_config(&mut self, width: u32, height: u32, iterations: u32) {
        self.width = width.into();
        self.height = height.into();
        self.iterations = iterations;
        if self.width > self.height * 1.5 {
            self.ratio = 2.0 / self.height;
        } else {
            self.ratio = 3.0 / self.width;
        }
    }
}

#[cfg(test)]
mod mandelbrot_functions_test {
    use super::*;

    #[test]
    fn calculate_ratio_test_1() {
        let calc = MandelbrotCalculator::new(500, 500, 100);
        assert_eq!(calc.ratio, 3.0 / 500.0);
    }

    #[test]
    fn calculate_ratio_test_2() {
        let calc = MandelbrotCalculator::new(2000, 500, 100);
        assert_eq!(calc.ratio, 2.0 / 500.0);
    }

    #[test]
    fn screen_to_complex_test_1() {
        let calc = MandelbrotCalculator::new(500, 500, 100);
        assert!((calc.screen_to_complex(250, 250) - Complex::<f64>::new(-0.5, 0.0)).norm() <= 0.01);
    }

    #[test]
    fn screen_to_complex_test_2() {
        let calc = MandelbrotCalculator::new(1920, 1080, 100);
        assert_eq!(
            calc.screen_to_complex(1920, 1080),
            Complex::<f64>::new(32.0 / 27.0, -1.0)
        )
    }
}
