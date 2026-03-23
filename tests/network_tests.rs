use nmt::network::{Link, Network};
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_load_network_from_valid_csv() {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(
        temp_file,
        "A,B,100\nB,C,200\nC,D,150\nD,A,120\nB,D,80"
    )
    .unwrap();

    let network = Network::from_csv(temp_file.path()).unwrap();

    assert_eq!(network.links.len(), 5);
    assert_eq!(
        network.links,
        vec![
            Link {
                node_a: "A".to_string(),
                node_b: "B".to_string(),
                capacity: 100,
            },
            Link {
                node_a: "B".to_string(),
                node_b: "C".to_string(),
                capacity: 200,
            },
            Link {
                node_a: "C".to_string(),
                node_b: "D".to_string(),
                capacity: 150,
            },
            Link {
                node_a: "D".to_string(),
                node_b: "A".to_string(),
                capacity: 120,
            },
            Link {
                node_a: "B".to_string(),
                node_b: "D".to_string(),
                capacity: 80,
            },
        ]
    );

    assert_eq!(network.adjacency_list["A"].len(), 2);
    assert_eq!(network.adjacency_list["B"].len(), 3);
    assert_eq!(network.adjacency_list["C"].len(), 2);
    assert_eq!(network.adjacency_list["D"].len(), 3);
}

#[test]
fn test_load_network_from_invalid_csv() {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "A,B,100\nB,C,invalid_capacity\nC,D,150").unwrap();

    let result = Network::from_csv(temp_file.path());
    assert!(result.is_err());
}

#[test]
fn test_add_link() {
    let mut network = Network::new();
    network.add_link("A".to_string(), "B".to_string(), 100);

    assert_eq!(network.links.len(), 1);
    assert_eq!(
        network.links[0],
        Link {
            node_a: "A".to_string(),
            node_b: "B".to_string(),
            capacity: 100,
        }
    );

    assert_eq!(network.adjacency_list["A"], vec![("B".to_string(), 100)]);
    assert_eq!(network.adjacency_list["B"], vec![("A".to_string(), 100)]);
}
