use std::collections::{HashMap};

impl Network {
    /// Applies traffic flow to the network by routing each traffic demand along the shortest path.
    /// Calculates the capacity used on each link and generates a report.
    pub fn apply_traffic_flow(&self) -> HashMap<(String, String), u32> {
        let mut link_usage: HashMap<(String, String), u32> = HashMap::new();

        for (node, &(ingress, egress)) in &self.traffic_data {
            if let Some((_, path)) = self.shortest_path("source", node) {
                // Add ingress traffic to the path
                for window in path.windows(2) {
                    if let [from, to] = window {
                        let link = (from.clone(), to.clone());
                        *link_usage.entry(link).or_insert(0) += ingress;
                    }
                }
            }

            if let Some((_, path)) = self.shortest_path(node, "sink") {
                // Add egress traffic to the path
                for window in path.windows(2) {
                    if let [from, to] = window {
                        let link = (from.clone(), to.clone());
                        *link_usage.entry(link).or_insert(0) += egress;
                    }
                }
            }
        }

        link_usage
    }

    /// Generates a report of the traffic flow, detailing the route of each traffic demand
    /// and the total demand for each link.
    pub fn generate_traffic_report(&self, link_usage: &HashMap<(String, String), u32>) -> String {
        let mut report = String::new();

        report.push_str("Traffic Flow Report:\n\n");
        for (link, usage) in link_usage {
            report.push_str(&format!(
                "Link {:?} -> {:?}: {} units\n",
                link.0, link.1, usage
            ));
        }

        report
    }
}
