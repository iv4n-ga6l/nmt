use nmt::network::{Network};
use std::collections::HashMap;

#[test]
fn test_analyze_worst_case_failure_single_link() {
    let mut network = Network::new();

    // Define a simple network
    network.add_link("source", "A", 10);
    network.add_link("A", "B", 5);
    network.add_link("B", "sink", 10);
    network.add_link("A", "sink", 15);

    // Define traffic data
    network.traffic_data.insert("A".to_string(), (5, 10));
    network.traffic_data.insert("B".to_string(), (10, 5));

    // Analyze worst case failure with a single link failure
    let worst_case = network.analyze_worst_case_failure(1);

    // Expected worst case failure
    let expected_worst_case = Some((
        vec![("A".to_string(), "B".to_string())], // Link whose failure causes the worst case
        5,                                         // Unroutable traffic
        1,                                         // Links over capacity
        2.0                                        // Max capacity ratio
    ));

    assert_eq!(worst_case, expected_worst_case);
}

#[test]
fn test_analyze_worst_case_failure_multiple_links() {
    let mut network = Network::new();

    // Define a more complex network
    network.add_link("source", "A", 10);
    network.add_link("A", "B", 5);
    network.add_link("B", "sink", 10);
    network.add_link("A", "sink", 15);
    network.add_link("source", "B", 20);

    // Define traffic data
    network.traffic_data.insert("A".to_string(), (10, 15));
    network.traffic_data.insert("B".to_string(), (15, 10));

    // Analyze worst case failure with up to 2 simultaneous link failures
    let worst_case = network.analyze_worst_case_failure(2);

    // Expected worst case failure
    let expected_worst_case = Some((
        vec![ 
            ("A".to_string(), "B".to_string()), 
            ("source".to_string(), "A".to_string())
        ], // Links whose failure causes the worst case
        25, // Unroutable traffic
        2,  // Links over capacity
        3.0 // Max capacity ratio
    ));

    assert_eq!(worst_case, expected_worst_case);
}
