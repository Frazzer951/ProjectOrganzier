# Release Steps

# 1. Update Changelog

```bash
git cliff --tag {{VERSION}} -o CHANGELOG.md
git commit  -a -m 'chore(release): prepare for release'
```

# 2. Cargo Release Action

```bash
cargo release {{VERSION} # Check if everything looks correct
cargo release {{VERSION} -x # Run Release
```
