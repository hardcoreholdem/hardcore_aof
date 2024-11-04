use crate::format::pretty_percent;
use std::fmt;

pub struct BetaAKoQQ {
    pub v1_ako: f64,
    pub v2_qq: f64,
}

impl fmt::Display for BetaAKoQQ {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "AKo:{},QQ:{}",
            pretty_percent(self.v1_ako),
            pretty_percent(self.v2_qq)
        )
    }
}
