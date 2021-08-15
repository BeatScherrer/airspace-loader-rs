// use serde::de::{self, Deserialize, Deserializer, Visitor};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename = "OPENAIP")]
pub struct OpenAip {
  #[serde(rename = "VERSION")]
  version: String,
  #[serde(rename = "DATAFORMAT")]
  data_format: f32,
  #[serde(rename = "AIRSPACES")]
  airspaces: Airspaces,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Airspaces {
  #[serde(rename = "ASP")]
  pub airspaces: Vec<Airspace>,
}

#[derive(Debug, Deserialize, Clone)]
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
  // #[serde(rename = "ALTLIMIT_TOP")]
  // pub upper: AltLimit,

  // #[serde(rename = "ALTLIMIT_BOTTOM")]
  // pub lower: AltLimit,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AltLimit {
  #[serde(rename = "ALT")]
  altitude: Altitude,
  reference: AltitudeReference,
}

#[derive(Debug, Deserialize, Clone)]
pub enum AltitudeReference {
  STD,
}

#[derive(Debug, Deserialize, Clone)]
pub enum Altitude {
  FL(u32),
  M(u32),
  FT(u32),
}

#[derive(Debug, Deserialize, Clone)]
pub enum AirspaceCategory {
  WAVE,
}

impl OpenAip {
  pub fn get_airspaces(&self) -> Vec<Airspace> {
    self.airspaces.airspaces.clone()
  }
}

// Implement Deserialization trait for OpenAip
// impl<'de> Deserialize<'de> for OpenAip {
//   fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//   where
//     D: Deserializer<'de>,
//   {
//     enum Field
//   }
// }
