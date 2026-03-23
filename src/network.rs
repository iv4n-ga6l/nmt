use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, PartialEq)]
pub struct Link {
    pub node_a: String,
    pub node_b: String,
    pub capacity: u32,
}

#[derive(Debug)]
pub struct Network {
    pub links: Vec<Link>,
    pub adjacency_list: HashMap<String, Vec<(String, u32)>>,
}

impl Network {
    /// Creates a new, empty Network.
    pub fn new() -> Self {
        Network {
            links: Vec::new(),
            adjacency_list: HashMap::new(),
        }
    }

    /// Loads a network from a CSV file.
    pub fn from_csv<P: AsRef<Path>>(path: P) -> Result<Self, io::Error> {
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);

        let mut network = Network::new();

        for (index, line) in reader.lines().enumerate() {
            let line = line?;
            let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();

            if parts.len() != 3 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Invalid format on line {}: {}", index + 1, line),
                ));
            }

            let node_a = parts[0].to_string();
            let node_b = parts[1].to_string();
            let capacity: u32 = parts[2].parse().map_err(|_| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Invalid capacity on line {}: {}", index + 1, parts[2]),
                )
            })?;

            network.add_link(node_a, node_b, capacity);
        }

        Ok(network)
    }

    /// Adds a link to the network.
    pub fn add_link(&mut self, node_a: String, node_b: String, capacity: u32) {
        let link = Link {
            node_a: node_a.clone(),
            node_b: node_b.clone(),
            capacity,
        };

        self.links.push(link);

        self.adjacency_list
            .entry(node_a.clone())
            .or_insert_with(Vec::new)
            .push((node_b.clone(), capacity));

        self.adjacency_list
            .entry(node_b)
            .or_insert_with(Vec::new)
            .push((node_a, capacity));
    }
}
