pub mod airspace;
pub mod open_aip;

// Assume OPEN aip data format for now since no other format is used and for POC
pub struct AirspaceLoader {}

use log::{debug, warn};
use serde_xml_rs;
use std::fs;
use std::io;
use std::io::Read;

pub fn load_from_file(file_name: &str) -> Result<open_aip::OpenAip, io::Error> {
  warn!("this is a test log");
  println!();

  // open the file, propagate file errors
  let f = Box::new(fs::File::open(&file_name)?);

  // TODO Pass format based on file extension
  load_from_reader(f)
}

pub fn load_from_reader(reader: Box<dyn Read>) -> Result<open_aip::OpenAip, io::Error> {
  debug!("loading from reader");
  let result: Result<open_aip::OpenAip, serde_xml_rs::Error> = serde_xml_rs::from_reader(reader);

  // Convert the rust error to io error
  match result {
    Ok(airspaces) => {
      println!("{:#?}", airspaces);
      Ok(airspaces)
    }
    Err(e) => {
      log::error!("{}", e);
      Err(io::Error::new(io::ErrorKind::InvalidData, e))
    }
  }
}

#[cfg(test)]
mod test_open_aip {

  //   use super::core::*;
  use super::*;

  static OPENAIP_EXAMPLE: &str = r#"
  <?xml version="1.0" encoding="UTF-8" standalone="yes"?>
  <OPENAIP VERSION="367810a0f94887bf79cd9432d2a01142b0426795" DATAFORMAT="1.1">
  <AIRSPACES>
  <ASP CATEGORY="WAVE">
    <VERSION>367810a0f94887bf79cd9432d2a01142b0426795</VERSION>
    <ID>18024</ID>
    <COUNTRY>DE</COUNTRY>
    <NAME>ALB-OST</NAME>
    <ALTLIMIT_TOP REFERENCE="STD">
      <ALT UNIT="FL">100</ALT>
    </ALTLIMIT_TOP>
    <ALTLIMIT_BOTTOM REFERENCE="STD">
      <ALT UNIT="FL">75</ALT>
    </ALTLIMIT_BOTTOM>
    <GEOMETRY>
      <POLYGON>9.6255555555556 48.5625, 9.8477777777778 48.659444444444, 9.8488590649036 48.671520456103, 9.8491357659723 48.676179034769, 9.8494899239756 48.688273884048, 9.8494860905378 48.692936043918, 9.8477777777778 48.705, 9.9372222222222 48.714166666667, 9.9380313248976 48.696291564689, 9.937831067577 48.678408978203, 9.9366228021006 48.660543713921, 9.9344092727478 48.64272052819, 9.9319444444444 48.630833333333, 9.6391666666667 48.496388888889, 9.6255555555556 48.5625</POLYGON>
    </GEOMETRY>
  </ASP>
  </AIRSPACES>
  </OPENAIP>
  "#;

  #[test]
  fn test_load_from_reader() {
    // load the file
    let airspaces = AirspaceLoader::load_from_reader(Box::new(OPENAIP_EXAMPLE.as_bytes())).unwrap();

    // Check the first airspaces element
    let airspace = &airspaces.get_airspaces()[0];

    println!("{:?}", airspace);

    // Check values
    assert_eq!(airspace.name, "ALB-OST");
    assert_eq!(
      airspace.version,
      String::from("367810a0f94887bf79cd9432d2a01142b0426795")
    );
    assert_eq!(airspace.country, "DE");

    // // lower limit
    // matches!(airspace.lower.reference, AltitudeReference::STD);
    // matches!(airspace.lower.altitude.unit, AltitudeUnit::FL);
    // assert_eq!(airspace.lower.altitude.value, 75);

    let polygon: Vec<(f32, f32)> = vec![
      (9.6255555555556, 48.5625),
      (9.8477777777778, 48.659444444444),
      (9.8488590649036, 48.671520456103),
      (9.8491357659723, 48.676179034769),
      (9.8494899239756, 48.688273884048),
      (9.8494860905378, 48.692936043918),
      (9.8477777777778, 48.705),
      (9.9372222222222, 48.714166666667),
      (9.9380313248976, 48.696291564689),
      (9.937831067577, 48.678408978203),
      (9.9366228021006, 48.660543713921),
      (9.9344092727478, 48.64272052819),
      (9.9319444444444, 48.630833333333),
      (9.6391666666667, 48.496388888889),
      (9.6255555555556, 48.5625),
    ];

    assert_eq!(airspace.geometry.points, polygon);
  }
}
