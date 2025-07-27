# Usage Examples

This document provides comprehensive examples of using the GitHub Documentation Download Tool with GitHub tree URLs.

## Basic Usage

### Download from GitHub Tree URLs
```bash
# Download docs from Rust documentation path
gh-docs-download --repo "https://github.com/rust-lang/rust/tree/main/src/doc"

# Download from TanStack Router docs
gh-docs-download --repo "https://github.com/TanStack/router/tree/main/docs"

# Download from specific branch documentation
gh-docs-download --repo "https://github.com/owner/repo/tree/feature-branch/docs"
```

### Different Documentation Structures
```bash
# Top-level docs directory
gh-docs-download --repo "https://github.com/microsoft/vscode/tree/main/docs"

# Nested documentation paths
gh-docs-download --repo "https://github.com/TanStack/router/tree/main/docs/router/eslint"

# Documentation in src directory
gh-docs-download --repo "https://github.com/rust-lang/rust/tree/main/src/doc"
```

## Output Control

### Custom Output Directory
```bash
# Specify custom output directory
gh-docs-download --repo "https://github.com/rust-lang/rust/tree/main/src/doc" --output ./rust-documentation

# Use absolute path
gh-docs-download --repo "https://github.com/microsoft/vscode/tree/main/docs" --output /home/user/docs/vscode

# Organize by project
gh-docs-download --repo "https://github.com/TanStack/router/tree/main/docs" --output ./docs/tanstack-router
```

### List Files Without Downloading
```bash
# Preview what would be downloaded
gh-docs-download --repo "https://github.com/rust-lang/rust/tree/main/src/doc" --list-only

# Check documentation size before downloading
gh-docs-download --repo "https://github.com/microsoft/vscode/tree/main/docs" --list-only

# Preview nested documentation structure
gh-docs-download --repo "https://github.com/TanStack/router/tree/main/docs/router/framework/react/guide" --list-only
```

## Advanced Options

### Control Recursion
```bash
# Disable recursive directory scanning (only scan the specified path level)
gh-docs-download --repo "https://github.com/owner/repo/tree/main/docs" --recursive false

# Enable recursive scanning (default behavior)
gh-docs-download --repo "https://github.com/owner/repo/tree/main/docs" --recursive true
```

### Working with Different Branches
```bash
# Download from main branch (most common)
gh-docs-download --repo "https://github.com/owner/repo/tree/main/docs"

# Download from develop branch
gh-docs-download --repo "https://github.com/owner/repo/tree/develop/documentation"

# Download from feature branch
gh-docs-download --repo "https://github.com/owner/repo/tree/feature-new-docs/docs"

# Download from specific tag/release
gh-docs-download --repo "https://github.com/owner/repo/tree/v1.0.0/docs"
```

## Real-World Scenarios

### Downloading Popular Project Documentation

#### TanStack Router Documentation
```bash
# Download main documentation
gh-docs-download --repo "https://github.com/TanStack/router/tree/main/docs" --output tanstack-router-docs

# Download specific framework guide
gh-docs-download --repo "https://github.com/TanStack/router/tree/main/docs/router/framework/react/guide" --output react-router-guide

# Download ESLint plugin docs
gh-docs-download --repo "https://github.com/TanStack/router/tree/main/docs/router/eslint" --output tanstack-eslint-docs
```

#### Rust Language Documentation
```bash
# Download Rust language documentation from specific path
gh-docs-download --repo "https://github.com/rust-lang/rust/tree/main/src/doc" --output rust-docs

# Expected files: README.md, various .md files in src/doc/
```

#### Open Source Project Documentation
```bash
# Download VS Code documentation
gh-docs-download --repo "https://github.com/microsoft/vscode/tree/main/docs" --output vscode-docs

# Download Node.js documentation
gh-docs-download --repo "https://github.com/nodejs/node/tree/main/doc" --output nodejs-docs
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
      - name: Download project docs from main branch
        run: |
          gh-docs-download --repo "https://github.com/${{ github.repository }}/tree/main/docs" \
            --output ./downloaded-docs
      
      - name: Download additional documentation paths
        run: |
          gh-docs-download --repo "https://github.com/${{ github.repository }}/tree/main/documentation" \
            --output ./downloaded-docs/additional
      
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
    stages {
        stage('Download Docs') {
            steps {
                sh '''
                    gh-docs-download --repo "https://github.com/org/repo/tree/main/docs" \
                        --output ./docs
                    
                    gh-docs-download --repo "https://github.com/org/repo/tree/main/api-docs" \
                        --output ./docs/api
                '''
            }
        }
        stage('Process Documentation') {
            steps {
                sh 'find ./docs -name "*.md" | wc -l'
            }
        }
    }
}
```

### Batch Processing Multiple Documentation Paths
```bash
#!/bin/bash
# Download docs from multiple specific documentation paths

declare -A doc_paths
doc_paths["tanstack-router"]="https://github.com/TanStack/router/tree/main/docs"
doc_paths["tanstack-eslint"]="https://github.com/TanStack/router/tree/main/docs/router/eslint"
doc_paths["rust-doc"]="https://github.com/rust-lang/rust/tree/main/src/doc"
doc_paths["vscode-docs"]="https://github.com/microsoft/vscode/tree/main/docs"

for name in "${!doc_paths[@]}"; do
    echo "Downloading docs: $name..."
    gh-docs-download --repo "${doc_paths[$name]}" --output "./docs/$name"
done
```

### Processing Documentation from Different Branches
```bash
#!/bin/bash
# Download documentation from multiple branches of the same repository

base_repo="https://github.com/owner/repo"
branches=("main" "develop" "v1.0" "feature-new-docs")

for branch in "${branches[@]}"; do
    echo "Downloading docs from branch: $branch..."
    gh-docs-download --repo "$base_repo/tree/$branch/docs" --output "./docs/$branch"
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

### Large Documentation Directories
```bash
# Preview first to estimate download size
gh-docs-download --repo "https://github.com/large-org/huge-repo/tree/main/docs" --list-only

# Use specific paths to avoid downloading entire documentation trees
gh-docs-download --repo "https://github.com/large-org/huge-repo/tree/main/docs/specific-section"
```

### Git Repository Size
```bash
# The tool uses shallow clone (--depth 1) for efficiency
# Large repositories with extensive history won't impact performance
# Only the specified documentation path is checked out using sparse checkout
```

## Troubleshooting Examples

### Common Issues and Solutions

#### Invalid Tree URL Format
```bash
# Problem: Invalid repository format error
# Solution: Ensure URL follows the tree format
# ❌ Wrong: https://github.com/owner/repo
# ❌ Wrong: https://github.com/owner/repo/docs
# ✅ Correct: https://github.com/owner/repo/tree/main/docs
gh-docs-download --repo "https://github.com/owner/repo/tree/main/docs"
```

#### Repository or Path Not Found
```bash
# Problem: Git operation failed or path doesn't exist
# Solution: Verify the tree URL in your browser first
# Check if the path exists at: https://github.com/owner/repo/tree/main/docs
gh-docs-download --repo "https://github.com/owner/repo/tree/main/docs" --list-only
```

#### No Documentation Found
```bash
# Problem: No documentation files found in the specified path
# Solution: Preview the path contents first
gh-docs-download --repo "https://github.com/owner/repo/tree/main/docs" --list-only

# The tool looks for files with documentation extensions (.md, .rst, etc.)
# Check if the path contains files matching documentation patterns
```

#### Git Not Available
```bash
# Problem: git command not found
# Solution: Install git on your system
# The tool requires git to be available in PATH for sparse checkout operations
```