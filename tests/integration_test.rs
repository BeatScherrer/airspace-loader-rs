use airspace_loader_rs::airspace;
use airspace_loader_rs::load_from_file;
use airspace_loader_rs::AirspaceSpecification;

#[test]
fn load_openaip_file() {
  let test_airspace = airspace::Airspace {
    version: Some(String::from("67810a0f94887bf79cd9432d2a01142b0426795")),
    id: Some(18024),
    country: String::from("DE"),
    name: String::from("ALB-OST"),
    upper: airspace::Altitude::FL(100, airspace::AltitudeReference::STD),
    lower: airspace::Altitude::FL(75, airspace::AltitudeReference::STD),
  };

  let mut test_airspaces: airspace::Airspaces = vec![];
  test_airspaces.push(test_airspace.clone());

  println!("{:?}", test_airspace);

  // Create the airspace loader to read from a file
  load_from_file("open_aip_test.xml", AirspaceSpecification::OpenAip).unwrap();
}
