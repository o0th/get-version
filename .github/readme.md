### Usage as github action

Get version from Cargo.toml

```yaml
# Checkout the repository
- uses: actions/checkout@v4

# Get version
- uses: o0th/get-version@0.2.1

# Read Cargo.toml
- run: echo "VERSION=$(get-version)" >> $GITHUB_ENV

# Pipe in
- run: echo "VERSION=$(cat Cargo.toml | get-version)" >> $GITHUB_ENV

# Use version
- run: echo "${{ env.VERSION }}"
```

### Usage as binary

Use the following command to download get-version binary from the GitHub releases:

```bash
curl -SLO https://github.com/o0th/get-version/releases/download/0.2.1/get-version
```

Execute the binary to get the version information from Cargo.toml:

```bash
./get-version
```

You can also pipe the contents of a file into get-version:

```bash
cat Cargo.toml | ./get-version
```
