# Troubleshooting

This guide covers common issues and their solutions when using the GitHub Documentation Download Tool.

## Common Issues

### 1. Invalid Repository Format

#### Symptoms
```
Error: Invalid repository format: Expected GitHub tree URL format
```

#### Possible Causes
- Using old repository slug format instead of tree URL
- Missing `/tree/branch/path` in the URL
- Incorrect URL structure

#### Solutions

**Use Correct Tree URL Format**
```bash
# ✅ Correct format:
gh-docs-download --repo "https://github.com/owner/repo/tree/main/docs"

# ❌ Incorrect formats:
gh-docs-download --repo "owner/repo"                           # Old slug format
gh-docs-download --repo "https://github.com/owner/repo"        # Missing tree path
gh-docs-download --repo "https://github.com/owner/repo/docs"   # Missing tree/branch
```

**Verify URL in Browser**
```bash
# Copy the tree URL from GitHub's web interface
# Navigate to the documentation directory in GitHub
# Copy the URL from the address bar
# Example: https://github.com/TanStack/router/tree/main/docs
```

### 2. Repository or Path Not Found

#### Symptoms
```
Error: Git operation failed: git clone --no-checkout --depth 1 <url>
fatal: repository 'https://github.com/owner/repo.git' not found
```

#### Possible Causes
- Repository doesn't exist
- Repository name is misspelled
- Documentation path doesn't exist
- Repository is private (our tool only works with public repos)

#### Solutions

**Verify Repository Exists**
```bash
# Check repository exists by visiting in browser
https://github.com/owner/repo

# Ensure exact case-sensitive spelling
gh-docs-download --repo "https://github.com/Microsoft/vscode/tree/main/docs"  # Correct case
```

**Verify Documentation Path Exists**
```bash
# Navigate to the path in GitHub web interface first
# Example: Check if https://github.com/owner/repo/tree/main/docs exists
# If the path doesn't exist, try common alternatives:

# Common documentation paths:
--repo "https://github.com/owner/repo/tree/main/docs"
--repo "https://github.com/owner/repo/tree/main/documentation"
--repo "https://github.com/owner/repo/tree/main/doc"
--repo "https://github.com/owner/repo/tree/main/guide"
```

**For Private Repositories**
```bash
# Our tool only works with public repositories
# Private repositories are not supported in the current git-only approach
# Use the GitHub web interface to download private repository documentation
```

### 3. No Documentation Found

#### Symptoms
```
Found 0 documentation files
```

#### Possible Causes
- The specified path contains no documentation files
- Files don't match documentation patterns
- Path exists but is empty

#### Solutions

**Preview Path Contents**
```bash
# Use --list-only to see what's in the path
gh-docs-download --repo "https://github.com/owner/repo/tree/main/docs" --list-only

# If no files are found, the path might not contain documentation files
```

**Check File Types**
```bash
# The tool looks for these file extensions:
# .md, .mdx, .markdown, .txt, .rst, .adoc, .asciidoc
# .org, .tex, .pdf, .html, .htm, .xml

# And these common names:
# readme, changelog, license, guide, tutorial, etc.

# If your files have different extensions, they won't be detected
```

**Try Different Paths**
```bash
# Try different common documentation locations:
gh-docs-download --repo "https://github.com/owner/repo/tree/main/docs" --list-only
gh-docs-download --repo "https://github.com/owner/repo/tree/main/documentation" --list-only
gh-docs-download --repo "https://github.com/owner/repo/tree/main/wiki" --list-only
```

### 4. Git Command Not Found

#### Symptoms
```
Error: No such file or directory (os error 2)
```

#### Possible Causes
- Git is not installed on the system
- Git is not in the PATH
- Git installation is corrupted

#### Solutions

**Install Git**
```bash
# On macOS (using Homebrew):
brew install git

# On Ubuntu/Debian:
sudo apt-get install git

# On Windows:
# Download from https://git-scm.com/download/win

# Verify installation:
git --version
```

**Check PATH**
```bash
# Verify git is in PATH
which git

# If not found, add git to your PATH or use full path
export PATH="/usr/local/bin:$PATH"
```

### 5. Permission Denied

#### Symptoms
```
Error: Permission denied (os error 13)
```

#### Possible Causes
- Insufficient permissions to create output directory
- Output directory is read-only
- Disk space issues

#### Solutions

**Check Output Directory Permissions**
```bash
# Ensure you have write permissions to the output directory
ls -la /path/to/output/directory

# Create output directory with proper permissions
mkdir -p ./my-docs
chmod 755 ./my-docs

# Use a different output directory
gh-docs-download --repo "https://github.com/owner/repo/tree/main/docs" --output ./alternative-path
```

**Check Disk Space**
```bash
# Verify sufficient disk space
df -h

# Clean up temporary files if needed
rm -rf /tmp/gh-docs-*
```

### 6. Large Repository Timeout

#### Symptoms
```
Error: Git operation timed out
```

#### Possible Causes
- Very large repository taking too long to clone
- Slow network connection
- Repository has large binary files

#### Solutions

**Use More Specific Paths**
```bash
# Instead of downloading entire docs directory:
gh-docs-download --repo "https://github.com/large-org/huge-repo/tree/main/docs"

# Try more specific subdirectories:
gh-docs-download --repo "https://github.com/large-org/huge-repo/tree/main/docs/api"
gh-docs-download --repo "https://github.com/large-org/huge-repo/tree/main/docs/guides"
```

**Preview Before Downloading**
```bash
# Check size before downloading
gh-docs-download --repo "https://github.com/large-org/huge-repo/tree/main/docs" --list-only
```

### 7. Branch or Tag Not Found

#### Symptoms
```
Error: pathspec 'branch-name' did not match any file(s) known to git
```

#### Possible Causes
- Branch or tag name doesn't exist
- Misspelled branch name
- Branch was recently deleted

#### Solutions

**Verify Branch Exists**
```bash
# Check available branches on GitHub web interface
# Common branch names: main, master, develop, dev

# Try common branch names:
--repo "https://github.com/owner/repo/tree/main/docs"
--repo "https://github.com/owner/repo/tree/master/docs"
--repo "https://github.com/owner/repo/tree/develop/docs"
```

**Use Specific Tags**
```bash
# Use specific version tags if available:
--repo "https://github.com/owner/repo/tree/v1.0.0/docs"
--repo "https://github.com/owner/repo/tree/release-1.2/docs"
```

## Getting Help

### Debug Information
When reporting issues, include:

```bash
# Tool version
gh-docs-download --version

# Git version
git --version

# Operating system
uname -a

# Command that failed
gh-docs-download --repo "https://github.com/owner/repo/tree/main/docs" --list-only
```

### Verbose Output
Unfortunately, the current version doesn't have verbose logging, but you can:

```bash
# Use --list-only first to verify the path works
gh-docs-download --repo "https://github.com/owner/repo/tree/main/docs" --list-only

# Check if the issue is with a specific path by trying a known working example
gh-docs-download --repo "https://github.com/TanStack/router/tree/main/docs/router/eslint" --list-only
```

### Known Working Examples
These URLs are known to work and can be used for testing:

```bash
# Small documentation directory
gh-docs-download --repo "https://github.com/TanStack/router/tree/main/docs/router/eslint" --list-only

# Medium-sized documentation
gh-docs-download --repo "https://github.com/TanStack/router/tree/main/docs" --list-only

# Rust documentation (larger)
gh-docs-download --repo "https://github.com/rust-lang/rust/tree/main/src/doc" --list-only
```

## Performance Tips

### For Large Documentation Directories
```bash
# Use --list-only first to estimate size
gh-docs-download --repo "https://github.com/large-org/huge-repo/tree/main/docs" --list-only

# Download specific sections instead of everything
gh-docs-download --repo "https://github.com/large-org/huge-repo/tree/main/docs/api"
gh-docs-download --repo "https://github.com/large-org/huge-repo/tree/main/docs/guides"
```

### For Slow Networks
```bash
# Start with smaller, more specific paths
gh-docs-download --repo "https://github.com/owner/repo/tree/main/docs/getting-started"

# Use list-only mode to verify before downloading
gh-docs-download --repo "https://github.com/owner/repo/tree/main/docs" --list-only
```

### Disk Space Management
```bash
# Clean up previous downloads
rm -rf ./downloads

# Use specific output directories
gh-docs-download --repo "https://github.com/owner/repo/tree/main/docs" --output ./project-docs
```