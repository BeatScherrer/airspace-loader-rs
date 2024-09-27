use airspace_loader_rs::airspace;
use airspace_loader_rs::load_from_file;
use airspace_loader_rs::AirspaceSpecification;

mod common;

#[test]
fn load_openaip_file() {
    common::init_logger();

    let polygon = vec![
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

    let test_airspace = airspace::Airspace {
        version: Some(String::from("67810a0f94887bf79cd9432d2a01142b0426795")),
        id: Some("18024".to_string()),
        country: String::from("DE"),
        name: String::from("ALB-OST"),
        upper: airspace::Altitude::FL(100.0, airspace::AltitudeReference::STD),
        lower: airspace::Altitude::FL(75.0, airspace::AltitudeReference::STD),
        geometry: polygon,
        category: airspace::AirspaceCategory::WAVE,
    };

    let mut test_airspaces: airspace::Airspaces = vec![];
    test_airspaces.push(test_airspace.clone());

    println!("{:?}", test_airspace);

    // Create the airspace loader to read from a file
    load_from_file(
        "tests/open_aip/open_aip_test.xml",
        AirspaceSpecification::OpenAip,
    )
    .unwrap();
}

#[test]
fn load_openaip_ch() {
    common::init_logger();

    load_from_file("tests/open_aip/ch_asp.xml", AirspaceSpecification::OpenAip).unwrap();
}
