
# Rust Pipeline Variables Manager

A Rust tool to manage GitLab pipeline variables across multiple projects. This tool allows you to update the pipeline variables with new content.
**Please be advised that all of the projects will be scanned if you provide Personal Access Token. To update variable at specific project please use Project Access Token.**
## Overview
This tool lists all the projects that can be accessed with the provided gitlab token and updates the given variable with the new content.
## Features

- List all projects in a GitLab organization. (if provided Personal Access Token)
- Check maintainer access for each project.
- Update the provided pipeline variable for projects with maintainer access.

## Setup

### Prerequisites

- Rust (latest stable version)
- Cargo (Rust's package manager)
- GitLab Personal Access Token / Project Access Token with `maintainer` or `owner` access

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/DanielKoklev/rust-gitlab-variables.git
   cd rust-gitlab-variables
   ```

2. Run the tool:
    
   ```bash
   cargo run -- -t <token> -v <variable-to-update> -n <new-content-for-the-variable>
   ```
3. Or you can build it with:
    ```bash
    cargo build 
    ```
    And then execute it via:
    ```bash
    ./targets/release/rust-gitlab-variables -t <token> -v <variable-to-update> -n <new-content-for-the-variable> 
    ```
