use nmt::network::{Network};
use std::collections::HashMap;

#[test]
fn test_apply_traffic_flow() {
    let mut network = Network::new();

    // Define a simple network
    network.add_link("source", "A", 10);
    network.add_link("A", "B", 5);
    network.add_link("B", "sink", 10);
    network.add_link("A", "sink", 15);

    // Define traffic data
    network.traffic_data.insert("A".to_string(), (5, 10));
    network.traffic_data.insert("B".to_string(), (10, 5));

    // Apply traffic flow
    let link_usage = network.apply_traffic_flow();

    // Expected link usage
    let mut expected_usage = HashMap::new();
    expected_usage.insert(("source".to_string(), "A".to_string()), 5); // Ingress to A
    expected_usage.insert(("A".to_string(), "B".to_string()), 10);    // Ingress to B
    expected_usage.insert(("B".to_string(), "sink".to_string()), 15); // Egress from B
    expected_usage.insert(("A".to_string(), "sink".to_string()), 10); // Egress from A

    assert_eq!(link_usage, expected_usage);
}

#[test]
fn test_generate_traffic_report() {
    let mut network = Network::new();

    // Define a simple network
    network.add_link("source", "A", 10);
    network.add_link("A", "B", 5);
    network.add_link("B", "sink", 10);
    network.add_link("A", "sink", 15);

    // Define traffic data
    network.traffic_data.insert("A".to_string(), (5, 10));
    network.traffic_data.insert("B".to_string(), (10, 5));

    // Apply traffic flow
    let link_usage = network.apply_traffic_flow();

    // Generate report
    let report = network.generate_traffic_report(&link_usage);

    // Expected report
    let expected_report = "Traffic Flow Report:\n\nLink \"source\" -> \"A\": 5 units\nLink \"A\" -> \"B\": 10 units\nLink \"B\" -> \"sink\": 15 units\nLink \"A\" -> \"sink\": 10 units\n";

    assert_eq!(report, expected_report);
}
