use nmt::network::{Link, Network};
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_shortest_path() {
    let mut network = Network::new();
    network.add_link("A".to_string(), "B".to_string(), 1);
    network.add_link("B".to_string(), "C".to_string(), 2);
    network.add_link("A".to_string(), "C".to_string(), 4);
    network.add_link("C".to_string(), "D".to_string(), 1);
    network.add_link("B".to_string(), "D".to_string(), 5);

    // Test shortest path from A to D
    let result = network.shortest_path("A", "D");
    assert_eq!(result, Some((4, vec!["A".to_string(), "B".to_string(), "C".to_string(), "D".to_string()])));

    // Test shortest path from A to C
    let result = network.shortest_path("A", "C");
    assert_eq!(result, Some((3, vec!["A".to_string(), "B".to_string(), "C".to_string()])));

    // Test shortest path from A to A (trivial case)
    let result = network.shortest_path("A", "A");
    assert_eq!(result, Some((0, vec!["A".to_string()])));

    // Test no path case
    let result = network.shortest_path("A", "E");
    assert_eq!(result, None);
}
