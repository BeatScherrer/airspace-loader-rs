/** this module provides deserialization of the open aip airspace specification.
 * The airspace loader crate uses an internal airspace structure to which the
 * open aip specific data structure must be converted to by implementing the
 * `From` trait.
 */
use crate::airspace;

// use serde::de::{self, Deserialize, Deserializer, Visitor};
use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "OPENAIP")]
pub struct OpenAip {
    #[serde(rename = "VERSION")]
    pub version: String,
    #[serde(rename = "DATAFORMAT")]
    pub data_format: f32,

    #[serde(rename = "AIRSPACES")]
    pub airspaces: Airspaces,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Airspaces {
    #[serde(rename = "ASP")]
    pub airspaces: Vec<Airspace>,
}

// TODO split these up
impl From<&Airspace> for airspace::Airspace {
    fn from(item: &Airspace) -> Self {
        airspace::Airspace {
            lower: match item.lower.altitude.unit {
                AltitudeUnit::FL => airspace::Altitude::FL(
                    item.lower.altitude.value,
                    match item.lower.reference {
                        AltitudeReference::STD => airspace::AltitudeReference::STD,
                        AltitudeReference::GND => airspace::AltitudeReference::GND,
                        AltitudeReference::MSL => airspace::AltitudeReference::MSL,
                    },
                ),
                AltitudeUnit::M => airspace::Altitude::M(
                    item.lower.altitude.value,
                    match item.lower.reference {
                        AltitudeReference::STD => airspace::AltitudeReference::STD,
                        AltitudeReference::GND => airspace::AltitudeReference::GND,
                        AltitudeReference::MSL => airspace::AltitudeReference::MSL,
                    },
                ),
                AltitudeUnit::F => airspace::Altitude::FT(
                    item.lower.altitude.value,
                    match item.lower.reference {
                        AltitudeReference::STD => airspace::AltitudeReference::STD,
                        AltitudeReference::GND => airspace::AltitudeReference::GND,
                        AltitudeReference::MSL => airspace::AltitudeReference::MSL,
                    },
                ),
            },
            upper: match item.upper.altitude.unit {
                AltitudeUnit::FL => airspace::Altitude::FL(
                    item.upper.altitude.value,
                    match item.upper.reference {
                        AltitudeReference::STD => airspace::AltitudeReference::STD,
                        AltitudeReference::GND => airspace::AltitudeReference::GND,
                        AltitudeReference::MSL => airspace::AltitudeReference::MSL,
                    },
                ),
                AltitudeUnit::M => airspace::Altitude::M(
                    item.upper.altitude.value,
                    match item.upper.reference {
                        AltitudeReference::STD => airspace::AltitudeReference::STD,
                        AltitudeReference::GND => airspace::AltitudeReference::GND,
                        AltitudeReference::MSL => airspace::AltitudeReference::MSL,
                    },
                ),
                AltitudeUnit::F => airspace::Altitude::FT(
                    item.upper.altitude.value,
                    match item.upper.reference {
                        AltitudeReference::STD => airspace::AltitudeReference::STD,
                        AltitudeReference::GND => airspace::AltitudeReference::GND,
                        AltitudeReference::MSL => airspace::AltitudeReference::MSL,
                    },
                ),
            },
            id: Some(item.id),
            version: Some(item.version.clone()),
            country: item.country.clone(),
            name: item.name.clone(),
            geometry: item.geometry.points.clone(),
            category: match item.category {
                AirspaceCategory::A => airspace::AirspaceCategory::A,
                AirspaceCategory::B => airspace::AirspaceCategory::B,
                AirspaceCategory::C => airspace::AirspaceCategory::C,
                AirspaceCategory::D => airspace::AirspaceCategory::D,
                AirspaceCategory::E => airspace::AirspaceCategory::E,
                AirspaceCategory::F => airspace::AirspaceCategory::F,
                AirspaceCategory::G => airspace::AirspaceCategory::G,
                AirspaceCategory::WAVE => airspace::AirspaceCategory::WAVE,
                AirspaceCategory::CTR => airspace::AirspaceCategory::D,
                AirspaceCategory::RMZ => airspace::AirspaceCategory::RMZ,
                AirspaceCategory::DANGER => airspace::AirspaceCategory::DANGER,
                AirspaceCategory::RESTRICTED => airspace::AirspaceCategory::RESTRICTED,
                AirspaceCategory::GLIDING => airspace::AirspaceCategory::GLIDING,
            },
        }
    }
}

impl From<Airspaces> for airspace::Airspaces {
    fn from(item: Airspaces) -> Self {
        item.airspaces
            .iter()
            .map(|airspace| airspace::Airspace::from(airspace))
            .collect()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Airspace {
    #[serde(rename = "CATEGORY")]
    pub category: AirspaceCategory,

    #[serde(rename = "VERSION")]
    pub version: String,

    #[serde(rename = "ID")]
    pub id: u32,

    #[serde(rename = "COUNTRY")]
    pub country: String,

    #[serde(rename = "NAME")]
    pub name: String,

    #[serde(rename = "ALTLIMIT_TOP")]
    pub upper: AltLimit,

    #[serde(rename = "ALTLIMIT_BOTTOM")]
    pub lower: AltLimit,

    #[serde(rename = "GEOMETRY")]
    pub geometry: Polygon,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AltLimit {
    #[serde(rename = "ALT")]
    altitude: Altitude,

    #[serde(rename = "REFERENCE")]
    reference: AltitudeReference,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AltitudeReference {
    STD,
    GND,
    MSL,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Altitude {
    #[serde(rename = "UNIT")]
    unit: AltitudeUnit,

    #[serde(rename = "$value")]
    value: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
enum AltitudeUnit {
    FL,
    F,
    M,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AirspaceCategory {
    WAVE,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    CTR,
    RMZ,
    DANGER,
    RESTRICTED,
    GLIDING,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Polygon {
    #[serde(rename = "POLYGON")]
    #[serde(deserialize_with = "points_from_string")]
    pub points: Vec<(f32, f32)>,
}

impl Polygon {
    pub fn new(points: Vec<(f32, f32)>) -> Self {
        Polygon { points }
    }
}

fn points_from_string<'de, D>(d: D) -> Result<Vec<(f32, f32)>, D::Error>
where
    D: Deserializer<'de>,
{
    let mut points: Vec<(f32, f32)> = vec![];

    let stringified: String = d.deserialize_string(StringVisitor).unwrap();
    let points_string: Vec<&str> = stringified.split(',').collect();

    for point_string in points_string {
        let point: Vec<f32> = point_string
            .trim()
            .split(' ')
            .map(|s| s.parse::<f32>().unwrap())
            .collect();

        let point_tuple: (f32, f32) = (point[0], point[1]);
        points.push(point_tuple);
    }

    Ok(points)
}

struct StringVisitor;

use std::fmt;

impl<'de> Visitor<'de> for StringVisitor {
    type Value = String;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(String::from(value))
    }
}
