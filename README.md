# Rust for Cybersecurity: Secret Scanner

This repository contains a Rust-based tool designed to detect secrets in code repositories. The goal of this project is to learn Rust while building a practical security tool.

## Features :

- Scan Repositories: Detect secrets in public GitHub repositories.

## Prerequisites

- Rust and Cargo installed.

## Installation

- Clone the repository:

		git clone https://github.com/nchatharoo/
		rust_for_cybersecurity.git
		
		cd rust_for_cybersecurity

Build the project:

    cargo build

## Usage

 - Scanning a GitHub Repository

		cargo run -- https://github.com/username/repo

- Scanning a Local Directory

		cargo run -- /path/to/directory

- Scanning a Local Git Repository

		cargo run -- /path/to/git/repo

## Custom Patterns

You can define custom patterns in a patterns.json file. The file should be in the root directory and have the following structure:
	
	{
	  "patterns": [
	    {
	      "name": "AWS Secret Key",
	      "regex": "AKIA[0-9A-Z]{16}"
	    },
	    {
	      "name": "Generic API Key",
	      "regex": "[a-zA-Z0-9]{32}"
	    }
	  ]
	}

## Future Plans

- Extend functionality to detect additional vulnerabilities.
- Improve performance and error handling.
