use std::collections::{HashMap};

impl Network {
    /// Analyzes the Worst Case Failure (WCF) for the network.
    /// Simulates the removal of each link, applies traffic demands, and evaluates the impact.
    /// Returns the link whose failure results in the worst-case scenario.
    pub fn analyze_worst_case_failure(&self) -> Option<((String, String), usize, usize, f64)> {
        let mut worst_case: Option<((String, String), usize, usize, f64)> = None;

        for link in self.links.keys() {
            // Clone the network and remove the current link
            let mut simulated_network = self.clone();
            simulated_network.links.remove(link);

            // Apply traffic flow to the modified network
            let link_usage = simulated_network.apply_traffic_flow();

            // Calculate metrics for the current failure scenario
            let mut unroutable_traffic = 0;
            let mut links_over_capacity = 0;
            let mut max_capacity_ratio = 0.0;

            for ((from, to), &usage) in &link_usage {
                if let Some(&capacity) = self.links.get(&(from.clone(), to.clone())) {
                    if usage > capacity {
                        links_over_capacity += 1;
                    }
                    let capacity_ratio = usage as f64 / capacity as f64;
                    if capacity_ratio > max_capacity_ratio {
                        max_capacity_ratio = capacity_ratio;
                    }
                } else {
                    unroutable_traffic += usage;
                }
            }

            // Update the worst-case scenario if this one is worse
            match &worst_case {
                Some((_, worst_unroutable, worst_over_capacity, worst_ratio)) => {
                    if unroutable_traffic > *worst_unroutable
                        || (unroutable_traffic == *worst_unroutable && links_over_capacity > *worst_over_capacity)
                        || (unroutable_traffic == *worst_unroutable && links_over_capacity == *worst_over_capacity && max_capacity_ratio > *worst_ratio)
                    {
                        worst_case = Some((link.clone(), unroutable_traffic, links_over_capacity, max_capacity_ratio));
                    }
                }
                None => {
                    worst_case = Some((link.clone(), unroutable_traffic, links_over_capacity, max_capacity_ratio));
                }
            }
        }

        worst_case
    }
}