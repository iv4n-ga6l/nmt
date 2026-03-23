use nmt::network::{Network};
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_load_traffic_data() {
    let mut network = Network::new();

    // Create a temporary file with traffic data
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(
        temp_file,
        "A 100 200\nB 150 250\nC 200 300\nD 50 75"
    )
    .unwrap();

    // Load traffic data from the file
    network.load_traffic_data(temp_file.path()).unwrap();

    // Verify the loaded traffic data
    assert_eq!(network.traffic_data.get("A"), Some(&(100, 200)));
    assert_eq!(network.traffic_data.get("B"), Some(&(150, 250)));
    assert_eq!(network.traffic_data.get("C"), Some(&(200, 300)));
    assert_eq!(network.traffic_data.get("D"), Some(&(50, 75)));
}

#[test]
fn test_load_traffic_data_invalid_format() {
    let mut network = Network::new();

    // Create a temporary file with invalid traffic data
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "A 100").unwrap(); // Missing egress value

    // Attempt to load traffic data from the file
    let result = network.load_traffic_data(temp_file.path());

    // Verify that an error is returned
    assert!(result.is_err());
}
