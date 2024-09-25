// TODO: user geo coordinates for airspace polygon
// How should round segments be represented generally?

use serde::Serialize;

pub type Airspaces = Vec<Airspace>;
pub type Polygon = Vec<Point>;
pub type Point = (f32, f32);

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Airspace {
    pub version: Option<String>,
    pub id: Option<u32>,
    pub country: String,
    pub name: String,
    pub upper: Altitude,
    pub lower: Altitude,
    pub geometry: Polygon,
    pub category: AirspaceCategory,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum AirspaceCategory {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    RMZ,
    TMZ,
    GLIDING,
    RESTRICTED,
    DANGER,
    WAVE,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Altitude {
    /// Flight level is always STD reference.
    FL(f32, AltitudeReference),
    /// Meters must specify a reference.
    M(f32, AltitudeReference),
    /// Feet must also specify a reference.
    FT(f32, AltitudeReference),
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum AltitudeReference {
    /// Standard pressure reference. FL implicitly is referenced to STD.
    STD,
    /// Mean Sea Level reference.
    MSL,
    /// Ground reference.
    GND,
}
