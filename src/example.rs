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
    <POLYGON>
    9.6255555555556 48.5625, 9.8477777777778 48.659444444444, 9.8488590649036 48.671520456103, 9.8491357659723 48.676179034769, 9.8494899239756 48.688273884048, 9.8494860905378 48.692936043918, 9.8477777777778 48.705, 9.9372222222222 48.714166666667, 9.9380313248976 48.696291564689, 9.937831067577 48.678408978203, 9.9366228021006 48.660543713921, 9.9344092727478 48.64272052819, 9.9319444444444 48.630833333333, 9.6391666666667 48.496388888889, 9.6255555555556 48.5625
    </POLYGON>
  </GEOMETRY>
</ASP>
</AIRSPACES>
</OPENAIP>
"#;

use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Config, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;

// use airspace_loader_rs::open_aip;

fn main() {
  // configure log4cxx logger
  let stdout = ConsoleAppender::builder()
    .encoder(Box::new(PatternEncoder::new("{d} - {m}{n}")))
    .build();

  let config = Config::builder()
    .appender(Appender::builder().build("stdout", Box::new(stdout)))
    .logger(Logger::builder().build("airspace_loader_rs", LevelFilter::Trace))
    .build(Root::builder().appender("stdout").build(LevelFilter::Debug))
    .unwrap();

  pretty_env_logger::init();

  log::trace!(target:"beat.open_aip", "Hello, World!");

  // // Create the airspace loader
  airspace_loader_rs::load_from_reader(Box::new(OPENAIP_EXAMPLE.as_bytes())).unwrap();
}
