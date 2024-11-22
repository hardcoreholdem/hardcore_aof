use crate::format::pretty_percent;
use std::fmt;

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
