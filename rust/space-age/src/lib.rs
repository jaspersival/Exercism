// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    earth_year: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration {
            earth_year: s as f64 / 31556952.0,
        }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        unimplemented!(
            "convert a duration ({d:?}) to the number of years on this planet for that duration"
        );
    }
}

pub struct Mercury;
impl Mercury {
    const EARTH_YEAR: f64 = 0.2408467;
}

pub struct Venus;
impl Venus {
    const EARTH_YEAR: f64 = 0.61519726;
}
pub struct Earth;
pub struct Mars;
impl Mars {
    const EARTH_YEAR: f64 = 1.8808158;
}

pub struct Jupiter;
impl Jupiter {
    const EARTH_YEAR: f64 = 11.862615;
}

pub struct Saturn;
impl Saturn {
    const EARTH_YEAR: f64 = 29.447498;
}

pub struct Uranus;
impl Uranus {
    const EARTH_YEAR: f64 = 84.016846;
}

pub struct Neptune;
impl Neptune {
    const EARTH_YEAR: f64 = 164.79132;
}

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        d.earth_year / 0.2408467
    }
}
impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        d.earth_year / 0.61519726
    }
}
impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        d.earth_year
    }
}
impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        d.earth_year / 1.8808158
    }
}
impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        d.earth_year / 11.862615
    }
}
impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        d.earth_year / 29.447498
    }
}
impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        d.earth_year / 84.016846
    }
}
impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        d.earth_year / 164.79132
    }
}
