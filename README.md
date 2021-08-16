# airspace-loader-rs

This library provides a loader for airspace data. Currently only [open aip](https://www.openaip.net/) is supported but can be extended for other formats.

# Usage

The airspace loader can be used in the following way:
```rust
let airspaces = airspace_loader_rs::AirspaceLoader::load_from_file("file_name.xml");
```