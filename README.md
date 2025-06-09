# Getting Started

A simple, lightweight Dynamic DNS (DDNS) updater using AWS Route53.  
This tool enables automatic DNS record updates when your public IP changes — ideal for home servers or dynamic IP setups.

## Features

- Automatically detects your current public IP address
- Updates a Route53 DNS A record to point to your current IP
- Simple configuration with the `setup` command
- Installs a systemd service on supported machines for continuous updates

## Prerequisites

- AWS credentials with permissions to modify Route53 records
- A registered domain with a hosted zone in AWS Route53

## Installation

You can install from the [latest release](https://github.com/rocketcamel/ddns-route53/releases/latest
)

or using `cargo`

```bash
cargo install --git https://github.com/rocketcamel/ddns-route53.git
```
(you will need pkg-config to do this)

## Usage

This program needs to be run as root to function correctly, since it uses systemd and is ran as the root user in the service. You can create your own timer or cron running at user level if this is not possible.

You need to have the `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`, and `AWS_REGION` environment variables set.
using `aws configure` or setting these manually is sufficient

Running `ddns-route53 setup` will interactively generate a config, and if possible create systemd timers
