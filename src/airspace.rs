pub mod core {
  pub type Airspaces = Vec<Airspace>;

  #[derive(Debug, Clone)]
  pub struct Airspace {
    pub version: String,
    pub id: u32,
    pub country: String,
    pub name: String,
    pub upper: Altitude,
    pub lower: Altitude,
    // pub geometry: Polygon,
  }

  // #[derive(Debug, Serialize, Deserialize)]
  // pub struct Altitude {
  //   pub value: u32,
  //   pub unit: AltitudeUnit,
  // }

  #[derive(Debug, Clone)]
  pub enum Altitude {
    FL(u32),
    M(u32),
    FT(u32),
  }

  #[derive(Debug, Clone)]
  pub enum AltitudeReference {
    STD,
  }

  pub struct Polygon {
    pub points: Vec<Point>,
  }

  #[derive(Debug)]
  pub struct Point(f32, f32);

  // struct StringVisitor;

  // impl StringVisitor {
  //   fn parse_points_from_string(string: &str) -> Vec<(f32, f32)> {
  //     let polygon: Vec<(f32, f32)> = vec![(0, 0), (1, 1)];

  //     let string_points: Vec<&str> = string.split(",").collect();

  //     println("{:?}", string_points);

  //     // parse points from the string_points
  //     for string_point in string_points {
  //       let point: Vec<&str> = string_point.split(" ").collect();
  //       println!("{:?}", point);
  //     }

  //     polygon
  //   }
  // }

  // impl<'de> Visitor<'de> for StringVisitor {
  //   // TODO abstract Point type with trait
  //   type Value = Vec<(f32, f32)>;

  //   fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
  //     formatter.write_str("a string")
  //   }

  //   fn visit_str<E>(self, value: &str) -> Result<Field, e>
  //   where
  //     E: de::Error
  //   {
  //     parse_points_from_string(value);
  //   }
  // }

  // impl<'de> Deserialize<'de> for Polygon {
  //   fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  //   where
  //     D: Deserializer<'de>
  //   {
  //     enum Field { Points }

  //     impl <'de> Deserialize<'de> for Points {
  //       fn deserialize<D>(deserializer:: D) -> Result<Field, D::Error>
  //       where
  //         D: Deserializer<'de>
  //       {
  //         struct FieldVisitor;

  //         impl<'de> Visitor<'de> for FieldVisitor {
  //           type Value = Field;

  //           fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
  //             formatter.write_str("`points`")
  //           }

  //           fn visit_str<E>(self, value: &str) -> Result<Field, E>
  //           where
  //             E: de::Error
  //           {
  //             match value {
  //               "points" => Ok(Field::Points),
  //               _ => Err(de::Error::unknown_field(value, FIELDS))
  //             }
  //           }
  //         }

  //         deserializer.deserialize_identifier(FieldVisitor)
  //       }
  //     }

  //     struct PolygonVisitor;

  //     impl<'de> Visitor<'de> for PolygonVisitor {
  //       type Value = Polygon;

  //       fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
  //         formatter.write_str("struct Polygon")
  //       }

  //       fn visit_seq<V>(self, mut seq: V) -> Result<Polygon, V::Error>
  //       where
  //         V: SeqAccess<'de>,
  //       {
  //         // TODO implement transformation here
  //       }
  //     }

  //     const FIELDS: &'static [&'static str] = &["points"];
  //     deserializer.deserialize_struct("Polygon", FIELDS, PolygonVisitor)
  //   }
  // }

  #[cfg(test)]
  mod test_airspace {

    //   #[test]
    //   fn test_polygon_deserialization() {
    //     use super::core::*;

    //     static POLYGON: &str = r#"<POLYGON>9.6255555555556 48.5625, 9.8477777777778 48.659444444444, 9.8488590649036 48.671520456103, 9.8491357659723 48.676179034769, 9.8494899239756 48.688273884048, 9.8494860905378 48.692936043918, 9.8477777777778 48.705, 9.9372222222222 48.714166666667, 9.9380313248976 48.696291564689, 9.937831067577 48.678408978203, 9.9366228021006 48.660543713921, 9.9344092727478 48.64272052819, 9.9319444444444 48.630833333333, 9.6391666666667 48.496388888889, 9.6255555555556 48.5625</POLYGON>"#;

    //     let polygon: Vec<(f32, f32)> = vec![
    //       (9.6255555555556, 48.5625),
    //       (9.8477777777778, 48.659444444444),
    //       (9.8488590649036, 48.671520456103),
    //       (9.8491357659723, 48.676179034769),
    //       (9.8494899239756, 48.688273884048),
    //       (9.8494860905378, 48.692936043918),
    //       (9.8477777777778, 48.705),
    //       (9.9372222222222, 48.714166666667),
    //       (9.9380313248976, 48.696291564689),
    //       (9.937831067577, 48.678408978203),
    //       (9.9366228021006, 48.660543713921),
    //       (9.9344092727478, 48.64272052819),
    //       (9.9319444444444, 48.630833333333),
    //       (9.6391666666667, 48.496388888889),
    //       (9.6255555555556, 48.5625),
    //     ];

    //     let result: Result<Polygon, serde_xml_rs::Error> = from_reader(POLYGON.as_bytes());

    //     assert_eq!(polygon, result);
    //   }
  }
}
