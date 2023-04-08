use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Complex {
    pub re: f32,
    pub im: f32
}

impl Complex {
    pub fn new(re: f32, im: f32) -> Complex {
        return Complex { re, im };
    }

    pub fn sq(&self) -> Complex {
        return Complex {
            re: self.re*self.re - self.im*self.im,
            im: 2.0*self.re*self.im
        };
    }

    pub fn abs(&self) -> f32{
        return (self.re*self.re + self.im*self.im).sqrt();
    }

    pub fn abs_sq(&self) -> f32{
        return self.re*self.re + self.im*self.im;
    }

    pub fn fuzzy_eq(self, other: Self) -> bool {
        float_fuzzy_eq(self.re, other.re) && float_fuzzy_eq(self.im, other.im)
    }
}

fn float_fuzzy_eq(lhs: f32, rhs: f32) -> bool {
    if lhs.is_sign_positive() ^ rhs.is_sign_positive() { // different signs, can't be fuzzy-equal
        return false
    }
    else {
        let lhs_i = lhs.abs().to_bits();
        let rhs_i = rhs.abs().to_bits();
        let ulps = lhs_i.abs_diff(rhs_i);
        ulps <= 2
    }
}

impl ops::Add<Complex> for Complex {
    type Output = Complex;
    fn add(self, rhs: Complex) -> Complex {
        return Complex::new(self.re + rhs.re, self.im + rhs.im);
    }
}
