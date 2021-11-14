pub type Airspaces = Vec<Airspace>;

#[derive(Debug, Clone, PartialEq)]
pub struct Airspace {
  pub version: Option<String>,
  pub id: Option<u32>,
  pub country: String,
  pub name: String,
  pub upper: Altitude,
  pub lower: Altitude,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Altitude {
  /// Flight level is always STD reference.
  FL(u32, AltitudeReference),
  /// Meters must specify a reference.
  M(u32, AltitudeReference),
  /// Feet must also specify a reference.
  FT(u32, AltitudeReference),
}

#[derive(Debug, Clone, PartialEq)]
pub enum AltitudeReference {
  /// Standard pressure reference. FL implicitly is referenced to STD.
  STD,
  /// Mean Sea Level reference.
  MSL,
  /// Ground reference.
  GND,
}

pub struct Polygon {
  /// Polygon of the airspace shape.
  pub points: Vec<Point>,
}

#[derive(Debug)]
pub struct Point(f32, f32);
