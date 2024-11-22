use crate::format::pretty_percent;
use crate::format::pretty_s;
use std::fmt;

#[derive(Clone, Copy)]
pub struct S(f64);

impl From<f64> for S {
    fn from(value: f64) -> Self {
        S(value)
    }
}

impl From<i32> for S {
    fn from(value: i32) -> Self {
        S(value as f64)
    }
}

impl Into<f64> for S {
    fn into(self) -> f64 {
        self.0
    }
}

impl S {
    pub fn midpoint(self, other: S) -> S {
        S((self.0 + other.0) / 2.0)
    }
}

impl fmt::Display for S {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", pretty_s(self.0))
    }
}

pub struct BetaAKoQQ {
    pub ako_1: f64,
    pub qq_2: f64,
}

impl fmt::Display for BetaAKoQQ {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "AKo:{},QQ:{}",
            pretty_percent(self.ako_1),
            pretty_percent(self.qq_2)
        )
    }
}

pub struct BetaAKoJJ {
    pub ako_1: f64,
    pub jj_2: f64,
}

impl fmt::Display for BetaAKoJJ {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "AKo:{},JJ:{}",
            pretty_percent(self.ako_1),
            pretty_percent(self.jj_2)
        )
    }
}
