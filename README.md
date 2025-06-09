# Getting Started

A simple, lightweight Dynamic DNS (DDNS) updater using AWS Route53.  
This tool enables automatic DNS record updates when your public IP changes — ideal for home servers or dynamic IP setups.

## Features

- Automatically detects your current public IP address
- Updates Route53 DNS A record to point to your current IP
- Simple configuration with the `setup` command
- Installs a systemd service on supported machines for continuous updates

## Prerequisites

- AWS credentials with permissions to modify Route53 records
- A registered domain with a hosted zone in AWS Route53

## Installation

You can install from the [Latest Release](https://github.com/rocketcamel/ddns-route53/releases/latest
)

or using `cargo`

```bash
cargo install --git https://github.com/rocketcamel/ddns-route53.git
```

(note this will compile the project on your local machine)
