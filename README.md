# Solana Native Auditing Introductory Labs

Welcome to Solana Native Security Labs! This repository is designed to equip you with a deep understanding of common vulnerabilities in Solana programs and best practices for securing them. Through hands-on labs, you'll learn how to identify and mitigate potential security threats in blockchain development, ensuring the integrity and security of your Solana applications.

## Why Security Matters

In the rapidly evolving world of blockchain technology, security stands as the cornerstone of trustworthy and reliable applications. By exploring these labs, you'll gain valuable insights into the intricacies of Solana's architecture and how to safeguard your programs against malicious attacks.

## Prerequisites

Before diving into the labs, ensure you have:
- A basic understanding of blockchain concepts.
- Familiarity with the Rust programming language.
- Docker installed if you prefer to use a containerized environment. 

## Getting Started

To embark on your journey with Solana Native Security Labs, start by cloning this repository to your local machine. Each lab is self-contained in its own directory under `labs/lab<num>`. For detailed instructions on how to navigate and complete a lab, refer to the README file within each lab's directory. Here's a quick overview of the steps you'll generally follow:
1. Navigate to the lab's directory.
2. Read the README file to understand the lab's objectives and vulnerabilities.
3. Open tests/mod.rs to view the lab's tests and follow `test_proper_flow` to understand the program's intended functionality.
4. Modify indicated sections within the `hack` test to exploit the program's vulnerabilities.
5. Tweak your code till you successfully exploit the vulnerability, indicated by both of the tests passing.

## Labs Overview

The labs cover a wide range of vulnerabilities, each designed to provide practical experience in identifying and mitigating security risks. Below is a list of the available labs, along with a brief description of what each lab covers:

- `hello-world`: A basic Solana program to get you started.
- `overflows`: Learn how to prevent overflow vulnerabilities.
- `unchecked-owner`: Understand the risks of unchecked ownership.
- `account-confusion`: Tackle deserialization vulnerabilities.
- `unchecked-program`: Investigate the dangers of unchecked programs.
- `unchecked-bumps`: Secure your programs against unchecked account bumps.
- `unchecked-mint`: Guard against vulnerabilities in token minting.
- `invoke-signed`: Explore signed invocation vulnerabilities.
- `unchecked-signer`: Learn how to handle unchecked signers properly.

Each lab includes two tests:
1. `test_proper_flow`: Demonstrates the intended operation of the program.
2. `hack`: A challenge to modify the program within specific markers to exploit vulnerabilities. This is your playground to apply what you've learned.

Utility functions are provided in some test modules to facilitate interaction with the lab exercises.

## Running the Labs

You can run the labs in your local environment or within a Docker container. For a seamless setup, we recommend using Docker. To start, simply run `docker-compose up` from the root of the repository. This will set up a development environment with all the necessary tools and expose VSCode on [http://127.0.0.1:8080](http://127.0.0.1:8080), ready for you to begin your exploration.

## Usage

To make your journey through the labs as smooth as possible, we've included a Justfile with convenient commands:
- `just test <lab_number>`: Verify the lab's functionality.
- `just hack <lab_number>`: Attempt to exploit the lab's vulnerabilities.

Replace `<lab_number>` with the number corresponding to the lab you wish to explore.

## Contribution Guidelines

We warmly welcome contributions! Whether it's suggesting new labs, enhancing existing ones, or reporting bugs, your input helps make Solana Native Security Labs better for everyone. Please refer to our contribution guidelines for more details on how to get involved.

<!-- TODO: Add contribution guide -->

## Security Disclaimer

The vulnerabilities demonstrated in these labs are for educational purposes only. We strongly encourage ethical use of this knowledge to improve security in the Solana ecosystem. Misuse of this information is discouraged and against the spirit of this project.

## License

This project is licensed under the [MIT License](LICENSE), fostering open collaboration and sharing of knowledge.