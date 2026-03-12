#[derive(Debug)]
pub struct Duration(u64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s)
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

macro_rules! orbital_period {
    ($(($t:ident, $p:expr)),+$(,)?) => {
        $(
            pub struct $t;

            impl Planet for $t {
                fn years_during(d: &Duration) -> f64 {
                    d.0 as f64 / 31_557_600.0 / $p
                }
            }
        )*
    };
}

orbital_period!(
    (Mercury, 0.2408467),
    (Venus, 0.61519726),
    (Earth, 1.0),
    (Mars, 1.8808158),
    (Jupiter, 11.862615),
    (Saturn, 29.447498),
    (Uranus, 84.016846),
    (Neptune, 164.79132),
);
