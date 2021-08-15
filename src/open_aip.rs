// use serde::de::{self, Deserialize, Deserializer, Visitor};
use serde::de::SeqAccess;
use serde::Deserialize;
use serde::Deserializer;

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

  #[serde(rename = "ALTLIMIT_TOP")]
  pub upper: AltLimit,

  #[serde(rename = "ALTLIMIT_BOTTOM")]
  pub lower: AltLimit,

  #[serde(rename = "GEOMETRY")]
  geometry: Polygon,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AltLimit {
  #[serde(rename = "ALT")]
  altitude: Altitude,

  #[serde(rename = "REFERENCE")]
  reference: AltitudeReference,
}

#[derive(Debug, Deserialize, Clone)]
pub enum AltitudeReference {
  STD,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Altitude {
  #[serde(rename = "UNIT")]
  unit: AltitudeUnit,

  #[serde(rename = "$value")]
  value: u32,
}

#[derive(Debug, Deserialize, Clone)]
enum AltitudeUnit {
  FL,
  FT,
  M,
}

#[derive(Debug, Deserialize, Clone)]
pub enum AirspaceCategory {
  WAVE,
}

#[derive(Debug, Clone)]
pub struct Polygon {
  // #[serde(rename = "POLYGON")]
  points: Vec<(f32, f32)>,
  // point: String,
}

impl Polygon {
  fn new(points: Vec<(f32, f32)>) -> Self {
    Polygon { points }
  }
}

impl OpenAip {
  pub fn get_airspaces(&self) -> Vec<Airspace> {
    self.airspaces.airspaces.clone()
  }
}

// ------------------------------------------------------------------------------
// serde Deserialization trait implementation
// ------------------------------------------------------------------------------

use serde::de::{self, Visitor};
use std::fmt;

// drive the visitor
impl<'de> Deserialize<'de> for Polygon {
  fn deserialize<D>(deserializer: D) -> Result<Polygon, D::Error>
  where
    D: Deserializer<'de>,
  {
    enum Field {
      Points,
    }

    impl<'de> Deserialize<'de> for Field {
      fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
      where
        D: Deserializer<'de>,
      {
        struct FieldVisitor;

        impl<'de> Visitor<'de> for FieldVisitor {
          type Value = Field;

          fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("`points`")
          }

          fn visit_str<E>(self, value: &str) -> Result<Field, E>
          where
            E: de::Error,
          {
            match value {
              "points" => Ok(Field::Points),
              _ => Err(de::Error::unknown_field(value, FIELDS)),
            }
          }
        }

        deserializer.deserialize_any(FieldVisitor)
      }
    }

    struct PolygonVisitor;

    // Define the polygon visitor
    impl PolygonVisitor {
      fn points_from_string(string: &str) -> Vec<(f32, f32)> {
        let point: (f32, f32);

        // let string_points: Vec<&str> = string.split(",").collect();
        string.split(",").map(|s| println!("{}", s));

        vec![(1.0, 2.0)]
      }
    }

    impl<'de> Visitor<'de> for PolygonVisitor {
      type Value = Polygon;

      fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("struct Polygon")
      }

      fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
      where
        E: de::Error,
      {
        // PolygonVisitor::points_from_string(value)
        // TODO map the string to (f32, f32)
        Ok(Polygon {
          points: vec![(3.0, 4.0)],
        })
      }

      fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E> {
        Ok(Polygon {
          points: vec![(3.0, 4.0)],
        })
      }

      fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
      where
        A: SeqAccess<'de>,
      {
        Ok(Polygon {
          points: vec![(3.0, 4.0)],
        })
      }
    }

    const FIELDS: &'static [&'static str] = &["points"];
    deserializer.deserialize_struct("Polygon", FIELDS, PolygonVisitor)
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
