# Solana Native Security Labs

Welcome to Solana Native Security Labs! This repository contains a collection of labs that demonstrate various vulnerabilities in Solana programs.

## Labs

The labs are located in the `labs/lab<num>` directory. Here is a list of available labs:

- `hello-world`: Demonstrates a basic Solana program.
- `overflows`: Illustrates potential overflow vulnerabilities.
- `unchecked-owner`: Explores vulnerabilities related to unchecked ownership.
- `account-confusion`: Focuses on deserialization vulnerabilities.
- `unchecked-program`: Examines vulnerabilities related to unchecked programs.
- `unchecked-bumps`: Demonstrates vulnerabilities related to unchecked account bumps.
- `unchecked-mint`: Explores vulnerabilities related to unchecked token minting.
- `invoke-signed`: Illustrates vulnerabilities related to unchecked signed invocations.
- `unchecked-signer`: Examines vulnerabilities related to unchecked signers.

Each lab serves as an example of a Solana vulnerability. Every lab consists of two tests:

1. `test_works_properly`: This test presents how the program is intended to work.
2. `hack`: This test checks if the program has been hacked. Hacker-students should only modify the text between the

 `// -- HACK --` and `//-- END HACK --` markers.

Some test modules include utility functions to facilitate interaction.

## Getting Started

TODO: Medium article

## Running the Labs

You can run the labs either on your local environment or using Docker. If you choose to use Docker, you can spin up the proper environment by running `docker-compose up`. This will host and expose VSCode on [http://127.0.0.1:8080](http://127.0.0.1:8080).

## Usage

This repository includes a Justfile that provides convenient commands for testing and attempting the hack for each lab. Here are the available commands:

- `just test <lab_number>`: Use this command to verify the lab.
- `just hack <lab_number>`: Use this command to attempt the hack.

Replace `<lab_number>` with the corresponding lab number.

## License

This project is licensed under the [MIT License](LICENSE).
