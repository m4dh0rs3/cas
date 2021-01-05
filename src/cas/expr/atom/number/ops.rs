use {
    super::{super::symbol::Symbol, Number},
    std::ops,
};

impl ops::Add for Number {
    type Output = Number;

    fn add(self, rhs: Number) -> Self::Output {
        Number(self.0 + rhs.0)
    }
}

impl ops::Sub for Number {
    type Output = Number;

    fn sub(self, rhs: Number) -> Self::Output {
        Number(self.0 - rhs.0)
    }
}

impl ops::Mul for Number {
    type Output = Number;

    fn mul(self, rhs: Number) -> Self::Output {
        Number(self.0 * rhs.0)
    }
}

impl ops::Div for Number {
    type Output = Number;

    fn div(self, rhs: Number) -> Self::Output {
        Number(self.0 / rhs.0)
    }
}

impl ops::Neg for Number {
    type Output = Number;

    fn neg(self) -> Self::Output {
        Number(-self.0)
    }
}

impl Number {
    pub(crate) fn abs(self) -> Number {
        Number(self.0.abs())
    }

    pub(crate) fn ceil(self) -> Number {
        Number(self.0.ceil())
    }

    pub(crate) fn floor(self) -> Number {
        Number(self.0.floor())
    }

    pub(crate) fn round(self) -> Number {
        Number(self.0.round())
    }

    pub(crate) fn trunc(self) -> Number {
        Number(self.0.trunc())
    }

    pub(crate) fn fract(self) -> Number {
        Number(self.0.fract())
    }

    pub(crate) fn signum(self) -> Number {
        Number(self.0.signum())
    }

    pub(crate) fn exp(self) -> Number {
        Number(self.0.exp())
    }

    pub(crate) fn fact(mut self) -> Number {
        if self.0 <= 0.0 || self.0 == 1.0 {
            return Number(1.0);
        }

        self * (self - Number(1.0)).fact()
    }

    pub(crate) fn ln(self) -> Number {
        Number(self.0.ln())
    }

    pub(crate) fn lg(self) -> Number {
        Number(self.0.log10())
    }

    pub(crate) fn sqrt(self) -> Number {
        Number(self.0.sqrt())
    }

    pub(crate) fn cbrt(self) -> Number {
        Number(self.0.cbrt())
    }

    pub(crate) fn sin(self) -> Number {
        Number(self.0.sin())
    }

    pub(crate) fn asin(self) -> Number {
        Number(self.0.asin())
    }

    pub(crate) fn sinh(self) -> Number {
        Number(self.0.sinh())
    }

    pub(crate) fn asinh(self) -> Number {
        Number(self.0.asinh())
    }

    pub(crate) fn cos(self) -> Number {
        Number(self.0.cos())
    }

    pub(crate) fn acos(self) -> Number {
        Number(self.0.acos())
    }

    pub(crate) fn cosh(self) -> Number {
        Number(self.0.cosh())
    }

    pub(crate) fn acosh(self) -> Number {
        Number(self.0.acosh())
    }

    pub(crate) fn tan(self) -> Number {
        Number(self.0.tan())
    }

    pub(crate) fn atan(self) -> Number {
        Number(self.0.atan())
    }

    pub(crate) fn tanh(self) -> Number {
        Number(self.0.tanh())
    }

    pub(crate) fn atanh(self) -> Number {
        Number(self.0.atanh())
    }

    // 2 args

    pub(crate) fn modulus(self, rhs: Number) -> Number {
        Number(self.0 % rhs.0)
    }

    pub(crate) fn pow(self, rhs: Number) -> Number {
        Number(self.0.powf(rhs.0))
    }

    pub(crate) fn equal(self, rhs: Number) -> Symbol {
        if self.0 == rhs.0 {
            Symbol(format!("true"))
        } else {
            Symbol(format!("false"))
        }
    }

    pub(crate) fn root(self, rhs: Number) -> Number {
        Number(self.0.powf(1.0 / rhs.0))
    }

    pub(crate) fn log(self, rhs: Number) -> Number {
        Number(self.0.log(rhs.0))
    }

    pub(crate) fn angle(self, rhs: Number) -> Number {
        Number(self.0.atan2(rhs.0))
    }

    // any number of args

    pub(crate) fn sum(args: Vec<Number>) -> Number {
        let mut sum = 0.0;

        for num in args {
            sum = sum + num.0;
        }

        Number(sum)
    }
}
