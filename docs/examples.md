# Usage Examples

This document provides comprehensive examples of using the GitHub Documentation Download Tool.

## Basic Usage

### Download from Repository Slug
```bash
# Download docs from Rust repository
gh-docs-download --repo rust-lang/rust

# Download docs from VS Code repository
gh-docs-download --repo microsoft/vscode
```

### Download from Full GitHub URL
```bash
# Download from full URL
gh-docs-download --repo https://github.com/tokio-rs/tokio

# Works with any GitHub URL format
gh-docs-download --repo https://github.com/facebook/react.git
```

## Output Control

### Custom Output Directory
```bash
# Specify custom output directory
gh-docs-download --repo rust-lang/rust --output ./rust-documentation

# Use absolute path
gh-docs-download --repo microsoft/vscode --output /home/user/docs/vscode
```

### List Files Without Downloading
```bash
# Preview what would be downloaded
gh-docs-download --repo rust-lang/rust --list-only

# Check documentation size before downloading
gh-docs-download --repo microsoft/vscode --list-only
```

## Authentication

### Using Environment Variable
```bash
# Set token via environment variable
export GITHUB_TOKEN=ghp_your_token_here
gh-docs-download --repo private-org/private-repo

# Token persists for session
gh-docs-download --repo another-org/another-repo
```

### Using Command Line Token
```bash
# Pass token directly
gh-docs-download --repo private-org/private-repo --token ghp_your_token_here

# Useful for CI/CD environments
gh-docs-download --repo org/repo --token $CI_GITHUB_TOKEN
```

## Advanced Options

### Force Git Clone Method
```bash
# Use git clone instead of API (no rate limits)
gh-docs-download --repo large-org/huge-repo --use-git

# Useful for repositories with many documentation directories
gh-docs-download --repo kubernetes/kubernetes --use-git --output k8s-docs
```

### Disable Recursive Directory Scanning
```bash
# Only scan top-level documentation directories
gh-docs-download --repo org/repo --recursive false
```

## Real-World Scenarios

### Downloading Popular Project Documentation

#### Rust Language Documentation
```bash
# Download Rust language documentation
gh-docs-download --repo rust-lang/rust --output rust-docs

# Expected directories: src/doc/, library/std/src/
# Expected files: README.md, CONTRIBUTING.md, etc.
```

#### React Documentation
```bash
# Download React documentation
gh-docs-download --repo facebook/react --output react-docs

# Expected directories: docs/
# Expected files: README.md, CHANGELOG.md, etc.
```

#### Kubernetes Documentation
```bash
# Large repository - use git method to avoid rate limits
gh-docs-download --repo kubernetes/kubernetes --use-git --output k8s-docs

# Expected directories: docs/, staging/src/k8s.io/kubectl/docs/
```

### CI/CD Integration

#### GitHub Actions Example
```yaml
name: Download Documentation
on: [push]
jobs:
  download-docs:
    runs-on: ubuntu-latest
    steps:
      - name: Download project docs
        run: |
          gh-docs-download --repo ${{ github.repository }} \
            --token ${{ secrets.GITHUB_TOKEN }} \
            --output ./downloaded-docs
      
      - name: Upload docs artifact
        uses: actions/upload-artifact@v3
        with:
          name: documentation
          path: ./downloaded-docs
```

#### Jenkins Pipeline Example
```groovy
pipeline {
    agent any
    environment {
        GITHUB_TOKEN = credentials('github-token')
    }
    stages {
        stage('Download Docs') {
            steps {
                sh '''
                    gh-docs-download --repo org/repo \
                        --token $GITHUB_TOKEN \
                        --output ./docs
                '''
            }
        }
    }
}
```

### Batch Processing Multiple Repositories
```bash
#!/bin/bash
# Download docs from multiple repositories

repos=(
    "rust-lang/rust"
    "microsoft/vscode"
    "facebook/react"
    "tokio-rs/tokio"
)

for repo in "${repos[@]}"; do
    echo "Downloading docs from $repo..."
    repo_name=$(echo $repo | cut -d'/' -f2)
    gh-docs-download --repo $repo --output "./docs/$repo_name"
done
```

## Output Structure Examples

### Typical Output Structure
```
downloads/
├── docs/
│   ├── README.md
│   ├── installation.md
│   └── user-guide/
│       ├── getting-started.md
│       └── advanced.md
├── documentation/
│   ├── API.md
│   └── reference/
│       └── functions.md
├── README.md
├── CHANGELOG.md
└── LICENSE
```

### Preserved Directory Structure
The tool maintains the original repository structure:
```
# Original repository structure:
repo/
├── docs/
│   └── user-guide.md
├── internal-docs/
│   └── architecture.md
└── README.md

# Downloaded structure:
downloads/
├── docs/
│   └── user-guide.md
├── internal-docs/
│   └── architecture.md
└── README.md
```

## Performance Considerations

### Large Repositories
```bash
# For repositories with extensive documentation
gh-docs-download --repo large-org/huge-repo --use-git

# Preview first to estimate download size
gh-docs-download --repo large-org/huge-repo --list-only
```

### Rate Limit Management
```bash
# Use authentication to increase rate limits
export GITHUB_TOKEN=your_token
gh-docs-download --repo org/repo

# Or use git method to bypass API limits entirely
gh-docs-download --repo org/repo --use-git
```

## Troubleshooting Examples

### Common Issues and Solutions

#### Rate Limited
```bash
# Problem: API rate limit exceeded
# Solution: Use authentication or git method
gh-docs-download --repo org/repo --token your_token
# OR
gh-docs-download --repo org/repo --use-git
```

#### Private Repository Access
```bash
# Problem: Repository not found (private repo)
# Solution: Provide authentication token
gh-docs-download --repo private-org/private-repo --token your_token
```

#### No Documentation Found
```bash
# Problem: No documentation directories found
# Solution: Check if repository has documentation
gh-docs-download --repo org/repo --list-only

# Some repositories may have docs in non-standard locations
# The tool looks for directories containing "doc" in the name
```