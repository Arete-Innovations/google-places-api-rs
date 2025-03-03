use serde::Deserialize;

#[derive(Debug, Deserialize, Copy, Clone, PartialEq)]
pub enum LocationBias {
    IpBias,
    Circular {
        radius: u32,
        latitude: f64,
        longitude: f64,
    },
    Rectangular {
        south: f64,
        west: f64,
        north: f64,
        east: f64,
    },
}

impl LocationBias {
    pub fn to_string(&self) -> String {
        match self {
            LocationBias::IpBias => "ipbias".to_string(),
            LocationBias::Circular {
                radius,
                latitude,
                longitude,
            } => {
                format!("circle:{}@{},{}", radius, latitude, longitude)
            }
            LocationBias::Rectangular {
                south,
                west,
                north,
                east,
            } => {
                format!("rectangle:{},{}|{},{}", south, west, north, east)
            }
        }
    }
}
