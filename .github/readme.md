### Usage as github action

Get version from Cargo.toml

```yaml
# Checkout the repository
- uses: actions/checkout@v4

# Get version
- uses: o0th/get-version@0.1.1

# Use version
- run: echo "Version: ${{ env.VERSION }}"
```

### Usage as binary

```bash
curl -SLO https://github.com/o0th/get-version/releases/download/0.1.1/get-version
./get-version
```
