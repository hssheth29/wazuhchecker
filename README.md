# Wazuhchecker

Wazuhchecker is a Rust-based tool designed to facilitate the installation and verification of the Wazuh agent on various Linux distributions. It automatically determines whether the Wazuh agent is installed and, if not, proceeds with the installation using the appropriate package manager for the detected Linux distribution.

## System Requirements

- Linux operating system (Debian-based, Red Hat-based, or Fedora)
- Rust programming environment
- Sudo or root access to install packages

## Installation

Clone the repository to your local machine:

```bash
git clone https://github.com/hssheth29/wazuhchecker.git
cd wazuhchecker
```

Build the project using Cargo, Rust's package manager and build system:

```bash
cargo build --release
```

This command compiles the project and generates an executable in `./target/release/`.

## Usage

Before running `wazuhchecker`, ensure you have sudo or root access as the tool needs to install packages.

To check for the Wazuh agent's installation status and install it if necessary, run:

```bash
sudo ./target/release/wazuhchecker
```

The tool will first check if the Wazuh agent is already installed. If it is not found, `wazuhchecker` will automatically attempt to install the agent.

## Supported Linux Distributions

- Debian-based distributions (using `apt-get`)
- Red Hat-based distributions (using `yum`)
- Fedora (using `dnf`)

## License

This project is licensed under the Apache License 2.0. See the LICENSE file for more details.
