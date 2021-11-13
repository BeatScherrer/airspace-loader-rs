use airspace_loader_rs::airspace::core;
use airspace_loader_rs::load_from_file;

#[test]
fn load_openaip_file() {
  let test_airspace = core::Airspace {
    version: String::from("67810a0f94887bf79cd9432d2a01142b0426795"),
    id: 18024,
    country: String::from("DE"),
    name: String::from("ALB-OST"),
    upper: core::Altitude::FL(100),
    lower: core::Altitude::FL(75),
  };

  let mut test_airspaces: core::Airspaces = vec![];
  test_airspaces.push(test_airspace.clone());

  println!("{:?}", test_airspace);

  // Create the airspace loader to read from a file
  load_from_file("open_aip_test.xml").unwrap();
}
