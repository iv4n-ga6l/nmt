# Network Modeling Toolkit (NMT)

The **Network Modeling Toolkit (NMT)** is a Rust-based library designed to analyze and simulate network behavior under various conditions. It provides tools for evaluating network performance, identifying bottlenecks, and simulating failure scenarios to assess network resilience.

## Features

- **Traffic Flow Simulation**: Simulate traffic flow across a network and evaluate link utilization.
- **Worst Case Failure Analysis**: Identify the combination of link failures that result in the worst-case scenario for the network.
- **Customizable Network Topology**: Define and modify network topologies with ease.

## Installation

To use this library, you need to have [Rust](https://www.rust-lang.org/) installed on your system. If you don't have Rust installed, you can install it using [rustup](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Once Rust is installed, clone this repository and build the project:

```bash
git clone https://github.com/iv4n-ga6l/nmt.git
cd nmt
cargo build
```

## Usage

### Example: Analyzing Worst Case Failure

Here's an example of how to use the library to analyze the worst-case failure scenario for a network:

```rust
use nmt::network::Network;
use std::collections::HashMap;

fn main() {
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

    println!("Worst case failure: {:?}", worst_case);
}
```

To run the example, save the code in a file (e.g., `main.rs`) and execute:

```bash
cargo run
```

## Testing

This repository includes a comprehensive test suite to ensure the correctness of the library. To run the tests, use the following command:

```bash
cargo test
```

### Example Tests

Here are some of the tests included in the repository:

- **Single Link Failure Analysis**: Verifies the worst-case failure when a single link is removed.
- **Multiple Link Failure Analysis**: Evaluates the worst-case scenario when multiple links fail simultaneously.

You can find the test cases in the `tests/network_tests.rs` file.

## Contributing

Contributions are welcome! If you'd like to contribute to this project, please follow these steps:

1. Fork the repository.
2. Create a new branch for your feature or bugfix.
3. Make your changes and commit them with clear commit messages.
4. Push your changes to your fork.
5. Open a pull request describing your changes.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contact

For any questions or feedback, please open an issue in the repository or contact the maintainer directly.
