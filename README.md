
# Rust Pipeline Variables Manager

A Rust tool to manage GitLab pipeline variables across multiple projects. This tool allows you to update the `GITLAB_ACCESS_TOKEN` variable for projects where you have maintainer access.
I also intend to add additional functionality for the user to be able to pass which variable should be updated, which will be more convenient.
## Overview

This tool interacts with the GitLab API to list all projects in an organization and update the `GITLAB_ACCESS_TOKEN` pipeline variable for projects where the authenticated user has maintainer access.

## Features

- List all projects in a GitLab organization.
- Check maintainer access for each project.
- Update the `GITLAB_ACCESS_TOKEN` pipeline variable for projects with maintainer access.

## Setup

### Prerequisites

- Rust (latest stable version)
- Cargo (Rust's package manager)
- GitLab Personal Access Token with `maintainer` or `owner` access

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/DanielKoklev/rust-gitlab-token.git
   cd rust-gitlab-token
   ```

2. Run the tool:
    
   ```bash
   cargo run -- --gitlab-token <paste-your-token-here>
   ```

