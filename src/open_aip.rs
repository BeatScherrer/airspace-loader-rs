// use serde::de::{self, Deserialize, Deserializer, Visitor};
use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "OPENAIP")]
pub struct OpenAip {
  #[serde(rename = "VERSION")]
  version: String,

  #[serde(rename = "DATAFORMAT")]
  data_format: f32,

  #[serde(rename = "AIRSPACES")]
  airspaces: Airspaces,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Airspaces {
  #[serde(rename = "ASP")]
  pub airspaces: Vec<Airspace>,
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
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Altitude {
  #[serde(rename = "UNIT")]
  unit: AltitudeUnit,

  #[serde(rename = "$value")]
  value: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
enum AltitudeUnit {
  FL,
  FT,
  M,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AirspaceCategory {
  WAVE,
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

impl OpenAip {
  pub fn get_airspaces(&self) -> Vec<Airspace> {
    self.airspaces.airspaces.clone()
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
