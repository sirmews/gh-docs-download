# Troubleshooting

This guide covers common issues and their solutions when using the GitHub Documentation Download Tool.

## Common Issues

### 1. Repository Not Found

#### Symptoms
```
Error: Repository not found or access denied
```

#### Possible Causes
- Repository doesn't exist
- Repository is private and requires authentication
- Incorrect repository format

#### Solutions

**Check Repository Exists**
```bash
# Verify the repository exists by visiting in browser
https://github.com/owner/repo

# Try with correct case-sensitive names
gh-docs-download --repo Microsoft/vscode  # Correct
gh-docs-download --repo microsoft/vscode  # May also work
```

**For Private Repositories**
```bash
# Use GitHub token
export GITHUB_TOKEN=your_token_here
gh-docs-download --repo private-org/private-repo

# Or pass token directly
gh-docs-download --repo private-org/private-repo --token your_token
```

**Check Repository Format**
```bash
# Correct formats:
gh-docs-download --repo owner/repo
gh-docs-download --repo https://github.com/owner/repo
gh-docs-download --repo https://github.com/owner/repo.git

# Incorrect formats:
gh-docs-download --repo github.com/owner/repo        # Missing protocol
gh-docs-download --repo owner-repo                   # Missing slash
```

### 2. API Rate Limiting

#### Symptoms
```
Rate limited or access denied. Consider using --token with a GitHub token.
```

#### Explanation
GitHub API has rate limits:
- **Unauthenticated**: 60 requests per hour per IP
- **Authenticated**: 5,000 requests per hour per user

#### Solutions

**Use Authentication**
```bash
# Get a GitHub token from https://github.com/settings/tokens
export GITHUB_TOKEN=ghp_your_token_here
gh-docs-download --repo owner/repo
```

**Use Git Clone Method**
```bash
# Bypass API entirely
gh-docs-download --repo owner/repo --use-git
```

**Wait and Retry**
```bash
# Check rate limit status
curl -H "Authorization: token your_token" https://api.github.com/rate_limit

# Wait for rate limit reset (shown in response)
```

### 3. No Documentation Found

#### Symptoms
```
No documentation directories found.
```

#### Possible Causes
- Repository has no documentation directories
- Documentation is in non-standard locations
- Repository structure doesn't match detection patterns

#### Solutions

**Check What the Tool Looks For**
The tool searches for directories containing "doc" in their name:
- `docs/`
- `documentation/`
- `doc/`
- `api-docs/`
- `user-docs/`

**Manual Verification**
```bash
# List all files to see what's available
gh-docs-download --repo owner/repo --list-only

# Check repository structure on GitHub
# Visit: https://github.com/owner/repo
```

**Alternative Approaches**
```bash
# Some repositories have docs in root directory
# The tool will still find README.md, CHANGELOG.md, etc.

# For repositories with non-standard structure,
# consider downloading the entire repository
git clone https://github.com/owner/repo.git
```

### 4. Download Failures

#### Symptoms
```
Error downloading path/to/file.md: Network error
Failed to download file.pdf
```

#### Possible Causes
- Network connectivity issues
- Large files timing out
- GitHub server issues
- Insufficient disk space

#### Solutions

**Check Network Connectivity**
```bash
# Test basic connectivity
ping github.com

# Test API access
curl -I https://api.github.com/repos/owner/repo
```

**Retry with Git Method**
```bash
# More reliable for large files
gh-docs-download --repo owner/repo --use-git
```

**Check Disk Space**
```bash
# Check available space
df -h .

# Preview download size first
gh-docs-download --repo owner/repo --list-only
```

### 5. Permission Denied

#### Symptoms
```
Permission denied (publickey)
Error: Git clone failed
```

#### Possible Causes
- SSH key not configured for git clone
- Insufficient permissions for output directory

#### Solutions

**For Git Clone Issues**
```bash
# The tool uses HTTPS, not SSH, so this shouldn't occur
# If it does, ensure git is properly installed
git --version

# Test git clone manually
git clone https://github.com/owner/repo.git
```

**For Output Directory Issues**
```bash
# Check permissions
ls -la downloads/

# Use different output directory
gh-docs-download --repo owner/repo --output ~/my-docs

# Create directory with proper permissions
mkdir -p ~/docs && chmod 755 ~/docs
gh-docs-download --repo owner/repo --output ~/docs
```

### 6. Large Repository Issues

#### Symptoms
```
Taking very long to complete
Memory usage high
Many API requests
```

#### Solutions

**Use Git Method**
```bash
# More efficient for large repositories
gh-docs-download --repo kubernetes/kubernetes --use-git
```

**Preview First**
```bash
# Check size before downloading
gh-docs-download --repo large-org/huge-repo --list-only
```

**Selective Downloading**
```bash
# Currently not supported, but you can:
# 1. Clone the repository manually
git clone --depth 1 https://github.com/large-org/huge-repo.git

# 2. Copy only documentation directories
cp -r huge-repo/docs/ ./my-docs/
```

## Environment Issues

### 7. Git Not Found

#### Symptoms
```
Error: Git clone failed: program not found
```

#### Solutions
```bash
# Install git
# Ubuntu/Debian:
sudo apt-get install git

# macOS:
brew install git

# Windows:
# Download from https://git-scm.com/download/win

# Verify installation
git --version
```

### 8. Rust/Cargo Issues

#### Symptoms
```
cargo: command not found
error: could not compile
```

#### Solutions
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Update Rust
rustup update

# Verify installation
cargo --version
rustc --version
```

## Token Issues

### 9. Invalid Token

#### Symptoms
```
Bad credentials
401 Unauthorized
```

#### Solutions
```bash
# Verify token format (should start with ghp_)
echo $GITHUB_TOKEN

# Test token manually
curl -H "Authorization: token $GITHUB_TOKEN" https://api.github.com/user

# Generate new token at https://github.com/settings/tokens
# Required scopes: repo (for private repos) or public_repo (for public repos)
```

### 10. Token Permissions

#### Symptoms
```
Repository not found (but repository exists)
403 Forbidden
```

#### Solutions
```bash
# Ensure token has correct scopes:
# - public_repo: for public repositories
# - repo: for private repositories

# Check token scopes
curl -H "Authorization: token $GITHUB_TOKEN" -I https://api.github.com/user
# Look for X-OAuth-Scopes header
```

## Performance Issues

### 11. Slow Downloads

#### Symptoms
- Downloads taking very long
- High memory usage
- Network timeouts

#### Solutions
```bash
# Use git method for better performance
gh-docs-download --repo owner/repo --use-git

# Check network speed
curl -o /dev/null -s -w "%{speed_download}\n" https://github.com/

# Monitor system resources
top
htop
```

### 12. High Memory Usage

#### Solutions
```bash
# Use git method (more memory efficient)
gh-docs-download --repo owner/repo --use-git

# Close other applications
# Monitor memory usage:
free -h  # Linux
vm_stat  # macOS
```

## Debugging

### Enable Debug Output

```bash
# Set debug logging
export RUST_LOG=debug
gh-docs-download --repo owner/repo

# Or for specific components
export RUST_LOG=reqwest=debug
gh-docs-download --repo owner/repo
```

### Manual Testing

```bash
# Test API access manually
curl -H "Authorization: token $GITHUB_TOKEN" \
  https://api.github.com/repos/owner/repo/contents

# Test git clone manually
git clone --depth 1 https://github.com/owner/repo.git test-clone
```

### Verbose Output

```bash
# Add verbose flag (if implemented)
gh-docs-download --repo owner/repo --verbose

# Or check what files would be downloaded
gh-docs-download --repo owner/repo --list-only
```

## Getting Help

### Check Version
```bash
gh-docs-download --version
```

### View Help
```bash
gh-docs-download --help
```

### Report Issues
When reporting issues, include:
1. Command used
2. Error message
3. Operating system
4. Rust version (`rustc --version`)
5. Repository being accessed (if public)

### Community Resources
- GitHub Issues: Report bugs and feature requests
- Documentation: Check docs/ directory for detailed guides
- Examples: See docs/examples.md for usage patterns

## Quick Fixes Summary

| Issue | Quick Fix |
|-------|-----------|
| Rate limited | Add `--token your_token` or `--use-git` |
| Repo not found | Check spelling, add token for private repos |
| No docs found | Use `--list-only` to see what's available |
| Download fails | Try `--use-git` method |
| Permission denied | Check output directory permissions |
| Git not found | Install git |
| Slow performance | Use `--use-git` for large repositories |
| High memory | Use `--use-git` method |
| Invalid token | Generate new token with correct scopes |