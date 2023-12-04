// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration{
    earth_days: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self{earth_days: s as f64 / 31557600f64}
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

macro_rules! years {
    ($s:ident, $fraction:literal) => {
        pub struct $s;
        impl Planet for $s {
            fn years_during(d: &Duration) -> f64 {
                d.earth_days / $fraction as f64
            }
        }
    };
}

years!{Earth, 1}
years!{Mercury, 0.2408467}
years!{Venus, 0.61519726}
years!{Mars, 1.8808158}
years!{Jupiter, 11.862615}
years!{Saturn, 29.447498}
years!{Uranus, 84.016846}
years!{Neptune, 164.79132}

