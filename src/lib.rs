pub mod airspace;
pub mod open_aip;

// Assume OPEN aip data format for now since no other format is used and for POC
pub struct AirspaceLoader {}

use log::{debug, error, info, warn};
use serde_xml_rs;
use std::fs;
use std::io;
use std::io::Read;

pub enum AirspaceSpecification {
  OpenAip,
}

pub fn load_from_file(
  file_name: &str,
  specification: AirspaceSpecification,
) -> Result<airspace::Airspaces, io::Error> {
  warn!("this is a test log");
  println!();

  // open the file, propagate file errors
  let f = Box::new(fs::File::open(&file_name)?);

  load_from_reader(f, specification)
}

pub fn load_from_reader(
  reader: impl Read,
  specification: AirspaceSpecification,
) -> Result<airspace::Airspaces, io::Error> {
  debug!("loading from reader");

  let _airspaces = airspace::Airspaces::new();

  // deserialize to the specified format
  match specification {
    AirspaceSpecification::OpenAip => {
      let result: Result<open_aip::Airspaces, serde_xml_rs::Error> =
        serde_xml_rs::from_reader(reader);

      info!("test");
      match result {
        Ok(value) => Ok(airspace::Airspaces::from(value)),
        Err(e) => {
          error!("{:#?}", e);
          Err(std::io::Error::new(std::io::ErrorKind::InvalidData, e))
        }
      }
    }
  }
}

#[cfg(test)]
mod test_open_aip {

  use super::*;
  use pretty_assertions::assert_eq;
  use std::sync::Once;

  static INIT: Once = Once::new();

  fn setup() {
    INIT.call_once(|| {
      pretty_env_logger::init();
    });
  }

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
    setup();

    // load the file
    let airspaces =
      load_from_reader(OPENAIP_EXAMPLE.as_bytes(), AirspaceSpecification::OpenAip).unwrap();

    // Check the first airspaces element
    let airspace = &airspaces[0];

    // TODO add polygon
    // create expected airspace structure

    // let polygon: Vec<(f32, f32)> = vec![
    //   (9.6255555555556, 48.5625),
    //   (9.8477777777778, 48.659444444444),
    //   (9.8488590649036, 48.671520456103),
    //   (9.8491357659723, 48.676179034769),
    //   (9.8494899239756, 48.688273884048),
    //   (9.8494860905378, 48.692936043918),
    //   (9.8477777777778, 48.705),
    //   (9.9372222222222, 48.714166666667),
    //   (9.9380313248976, 48.696291564689),
    //   (9.937831067577, 48.678408978203),
    //   (9.9366228021006, 48.660543713921),
    //   (9.9344092727478, 48.64272052819),
    //   (9.9319444444444, 48.630833333333),
    //   (9.6391666666667, 48.496388888889),
    //   (9.6255555555556, 48.5625),
    // ];

    let expected = airspace::Airspace {
      version: Some(String::from("367810a0f94887bf79cd9432d2a01142b0426795")),
      id: Some(18024),
      country: String::from("DE"),
      name: String::from("ALB_OST"),
      upper: airspace::Altitude::FL(100, airspace::AltitudeReference::STD),
      lower: airspace::Altitude::FL(75, airspace::AltitudeReference::STD),
    };

    assert_eq!(airspace, &expected);
  }
}
