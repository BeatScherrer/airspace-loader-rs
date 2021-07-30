pub mod open_aip {

  use log::warn;
  use serde::Deserialize;
  use std::io::Error;
  use std::io::Read;

  #[derive(Debug, Deserialize)]
  pub struct OpenAIP {
    #[serde(rename = "AIRSPACES")]
    pub airspaces: Airspaces,
  }

  #[derive(Debug, Deserialize)]
  pub struct Airspaces {
    #[serde(rename = "ASP")]
    pub airspaces: Vec<Airspace>,
  }

  #[derive(Debug, Deserialize)]
  pub struct Airspace {
    #[serde(rename = "VERSION")]
    pub version: String,

    #[serde(rename = "ID")]
    pub id: String,

    #[serde(rename = "COUNTRY")]
    pub country: String,

    #[serde(rename = "NAME")]
    pub name: String,

    #[serde(rename = "ALTLIMIT_TOP")]
    pub upper: AltitudeLimit,

    #[serde(rename = "ALTLIMIT_BOTTOM")]
    pub lower: AltitudeLimit,
    // #[serde(rename = "GEOMETRY")]
    // pub geometry: Polygon,
  }

  #[derive(Debug, Deserialize)]
  pub struct AltitudeLimit {
    #[serde(rename = "REFERENCE")]
    pub reference: AltitudeReference,

    #[serde(rename = "ALT")]
    pub altitude: Altitude,
  }

  #[derive(Debug, Deserialize)]
  pub struct Altitude {
    #[serde(rename = "$value")]
    pub value: u32,
    #[serde(rename = "UNIT")]
    pub unit: AltitudeUnit,
  }

  #[derive(Debug, Deserialize)]
  #[serde(rename = "REFERENCE")]
  pub enum AltitudeReference {
    STD,
  }

  #[derive(Debug, Deserialize)]
  pub enum AltitudeUnit {
    FL,
    M,
    FT,
  }

  // #[derive(Debug, Deserialize)]
  // pub struct Polygon {
  //   #[serde(rename = "$value")]
  //   pub points: String,
  // }

  pub struct AirspaceLoader {}

  use serde_xml_rs::from_reader;
  use std::fs;
  use std::io;

  impl AirspaceLoader {
    pub fn new() -> AirspaceLoader {
      AirspaceLoader {}
    }

    pub fn load_from_file(&self, file_name: &str) -> Result<Airspaces, io::Error> {
      warn!("this is a test log");

      // propagate the error to the caller
      let f = Box::new(fs::File::open(&file_name).unwrap());

      AirspaceLoader::load_from_reader(f)
    }

    fn load_from_reader(reader: Box<dyn Read>) -> Result<Airspaces, io::Error> {
      let result: Result<OpenAIP, serde_xml_rs::Error> = from_reader(reader);

      match result {
        Ok(v) => Ok(v.airspaces),
        Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, e)),
      }
    }

    pub fn parse_polygon_from_string(string: &str) -> Vec<(f32, f32)> {
      let polygon: Vec<(f32, f32)> = vec![];

      let points: Vec<&str> = string.split(",").collect();
      println!("{:#?}", points);

      for item in points {
        let point: Vec<&str> = item.split(" ").collect();
        println!("{:#?}", point);
      }

      polygon
    }
  }
}

#[cfg(test)]
mod test_open_aip {
  use super::open_aip::*;

  static OPENAIP_EXAMPLE: &str = "
<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>
<OPENAIP VERSION=\"367810a0f94887bf79cd9432d2a01142b0426795\" DATAFORMAT=\"1.1\">
<AIRSPACES>
<ASP CATEGORY=\"WAVE\">
  <VERSION>367810a0f94887bf79cd9432d2a01142b0426795</VERSION>
  <ID>18024</ID>
  <COUNTRY>DE</COUNTRY>
  <NAME>ALB-OST</NAME>
  <ALTLIMIT_TOP REFERENCE=\"STD\">
    <ALT UNIT=\"FL\">100</ALT>
  </ALTLIMIT_TOP>
  <ALTLIMIT_BOTTOM REFERENCE=\"STD\">
    <ALT UNIT=\"FL\">75</ALT>
  </ALTLIMIT_BOTTOM>
  <GEOMETRY>
    <POLYGON>9.6255555555556 48.5625, 9.8477777777778 48.659444444444, 9.8488590649036 48.671520456103, 9.8491357659723 48.676179034769, 9.8494899239756 48.688273884048, 9.8494860905378 48.692936043918, 9.8477777777778 48.705, 9.9372222222222 48.714166666667, 9.9380313248976 48.696291564689, 9.937831067577 48.678408978203, 9.9366228021006 48.660543713921, 9.9344092727478 48.64272052819, 9.9319444444444 48.630833333333, 9.6391666666667 48.496388888889, 9.6255555555556 48.5625</POLYGON>
  </GEOMETRY>
</ASP>
</AIRSPACES>
</OPENAIP>
";

  #[test]
  fn test_load_openaip() {
    // load the file
    let loader = AirspaceLoader::new();
    let airspaces = loader.load_from_file(OPENAIP_EXAMPLE);
    let airspaces = match airspaces {
      Ok(airspaces) => airspaces,
      Err(e) => panic!("{}", e),
    };

    let airspace = &airspaces[0];

    // Check values
    assert_eq!(airspace.name, "ALB-OST");
    assert_eq!(
      airspace.version,
      String::from("367810a0f94887bf79cd9432d2a01142b0426795")
    );
    assert_eq!(airspace.country, "DE");

    // upper limit
    matches!(airspace.upper.reference, AltitudeReference::STD);
    matches!(airspace.upper.altitude.unit, AltitudeUnit::FL);
    assert_eq!(airspace.upper.altitude.value, 100);

    // lower limit
    matches!(airspace.lower.reference, AltitudeReference::STD);
    matches!(airspace.lower.altitude.unit, AltitudeUnit::FL);
    assert_eq!(airspace.lower.altitude.value, 75);

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

    // assert_eq!(airspace.polygon, polygon);
  }
}
