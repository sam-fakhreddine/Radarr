---
inclusion: always
---

# Always Lint and Fix

**All code MUST pass linting and formatting checks before commit.**

## Required Tools

### 1. rustfmt (Code Formatting)
```bash
# Install
rustup component add rustfmt

# Format all code
cargo fmt

# Check formatting without modifying
cargo fmt -- --check

# Format on save (configure in your editor)
```

### 2. Clippy (Linting)
```bash
# Install
rustup component add clippy

# Run clippy
cargo clippy

# Treat warnings as errors
cargo clippy -- -D warnings

# Fix automatically where possible
cargo clippy --fix
```

### 3. cargo check (Fast Compilation Check)
```bash
# Quick syntax and type check
cargo check

# Check all targets
cargo check --all-targets
```

### 4. cargo test (Run Tests)
```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

### 5. cargo audit (Security Audit)
```bash
# Install
cargo install cargo-audit

# Check for security vulnerabilities
cargo audit

# Fix vulnerabilities
cargo audit fix
```

## Clippy Configuration

### Cargo.toml Lints
```toml
[lints.rust]
unsafe_code = "forbid"
missing_docs = "warn"

[lints.clippy]
# Enable all clippy lints
all = "warn"
pedantic = "warn"
nursery = "warn"

# Deny dangerous patterns
unwrap_used = "deny"
expect_used = "deny"
panic = "deny"
todo = "deny"
unimplemented = "deny"

# Additional restrictions
missing_errors_doc = "warn"
missing_panics_doc = "warn"
```

### clippy.toml (Project Root)
```toml
# Cognitive complexity threshold
cognitive-complexity-threshold = 15

# Avoid false positives
avoid-breaking-exported-api = true

# Documentation
missing-docs-in-crate-items = true
```

## rustfmt Configuration

### rustfmt.toml (Project Root)
```toml
# Edition
edition = "2021"

# Line width
max_width = 100
comment_width = 80

# Imports
imports_granularity = "Crate"
group_imports = "StdExternalCrate"

# Formatting
use_small_heuristics = "Default"
fn_single_line = false
where_single_line = false

# Trailing comma
trailing_comma = "Vertical"

# Chains
chain_width = 60
```

## Pre-commit Workflow

### Manual Checks
```bash
# 1. Format code
cargo fmt

# 2. Run clippy
cargo clippy --all-targets -- -D warnings

# 3. Run tests
cargo test

# 4. Security audit
cargo audit

# 5. Build
cargo build --release
```

### Git Pre-commit Hook
Create `.git/hooks/pre-commit`:
```bash
#!/bin/bash

echo "Running pre-commit checks..."

# Format check
echo "Checking formatting..."
cargo fmt -- --check
if [ $? -ne 0 ]; then
    echo "❌ Code is not formatted. Run 'cargo fmt' to fix."
    exit 1
fi

# Clippy
echo "Running clippy..."
cargo clippy --all-targets -- -D warnings
if [ $? -ne 0 ]; then
    echo "❌ Clippy found issues. Fix them before committing."
    exit 1
fi

# Tests
echo "Running tests..."
cargo test --quiet
if [ $? -ne 0 ]; then
    echo "❌ Tests failed. Fix them before committing."
    exit 1
fi

echo "✅ All checks passed!"
exit 0
```

Make it executable:
```bash
chmod +x .git/hooks/pre-commit
```

## Common Clippy Warnings and Fixes

### 1. Unnecessary Clones
```rust
// ❌ Bad
fn process(s: String) -> String {
    s.clone()
}

// ✅ Good
fn process(s: String) -> String {
    s
}
```

### 2. Needless Borrow
```rust
// ❌ Bad
let x = &vec![1, 2, 3];
for item in x.iter() { }

// ✅ Good
let x = vec![1, 2, 3];
for item in &x { }
```

### 3. Redundant Field Names
```rust
// ❌ Bad
Movie {
    id: id,
    title: title,
}

// ✅ Good
Movie {
    id,
    title,
}
```

### 4. Explicit Returns
```rust
// ❌ Bad
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

// ✅ Good
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### 5. Match on Bool
```rust
// ❌ Bad
match is_valid {
    true => do_something(),
    false => do_other(),
}

// ✅ Good
if is_valid {
    do_something()
} else {
    do_other()
}
```

### 6. Single Match
```rust
// ❌ Bad
match result {
    Some(x) => println!("{}", x),
    None => {},
}

// ✅ Good
if let Some(x) = result {
    println!("{}", x);
}
```

### 7. Unnecessary Unwrap
```rust
// ❌ Bad
let value = option.unwrap();

// ✅ Good
let value = option?;
// or
let value = option.ok_or(Error::Missing)?;
```

### 8. Large Enum Variants
```rust
// ❌ Bad
enum Response {
    Success(LargeStruct),  // 1KB
    Error(String),         // 24 bytes
}

// ✅ Good
enum Response {
    Success(Box<LargeStruct>),
    Error(String),
}
```

### 9. Inefficient String Building
```rust
// ❌ Bad
let mut s = String::new();
s = s + "hello";
s = s + " world";

// ✅ Good
let mut s = String::new();
s.push_str("hello");
s.push_str(" world");
```

### 10. Needless Collect
```rust
// ❌ Bad
let sum: i32 = vec.iter()
    .map(|x| x * 2)
    .collect::<Vec<_>>()
    .iter()
    .sum();

// ✅ Good
let sum: i32 = vec.iter()
    .map(|x| x * 2)
    .sum();
```

## Editor Integration

### VS Code
```json
{
    "rust-analyzer.checkOnSave.command": "clippy",
    "rust-analyzer.checkOnSave.extraArgs": ["--", "-D", "warnings"],
    "editor.formatOnSave": true,
    "[rust]": {
        "editor.defaultFormatter": "rust-lang.rust-analyzer"
    }
}
```

### IntelliJ IDEA / RustRover
- Settings → Languages & Frameworks → Rust → Rustfmt
  - ✅ Run rustfmt on Save
- Settings → Languages & Frameworks → Rust → External Linters
  - ✅ Run external linter to analyze code on the fly
  - Select: Clippy

### Vim/Neovim
```vim
" Using rust.vim
let g:rustfmt_autosave = 1
let g:rust_clip_command = 'cargo clippy'
```

## CI/CD Integration

### GitHub Actions
```yaml
name: Rust CI

on: [push, pull_request]

jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          components: rustfmt, clippy
          
      - name: Check formatting
        run: cargo fmt -- --check
        
      - name: Run clippy
        run: cargo clippy --all-targets -- -D warnings
        
      - name: Run tests
        run: cargo test
        
      - name: Security audit
        run: |
          cargo install cargo-audit
          cargo audit
```

## Fixing Clippy Issues

### Automatic Fixes
```bash
# Fix all auto-fixable issues
cargo clippy --fix

# Fix and allow dirty working directory
cargo clippy --fix --allow-dirty

# Fix and allow staged changes
cargo clippy --fix --allow-staged
```

### Manual Fixes
1. Read the clippy message carefully
2. Understand why it's a problem
3. Apply the suggested fix
4. Re-run clippy to verify
5. Run tests to ensure nothing broke

### Suppressing Warnings (Use Sparingly)
```rust
// Suppress for a single item
#[allow(clippy::too_many_arguments)]
fn complex_function(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) { }

// Suppress for a module
#![allow(clippy::module_name_repetitions)]

// Suppress for a block
#[allow(clippy::cast_possible_truncation)]
{
    let x = value as u32;
}
```

**Only suppress when:**
- You have a good reason
- You've documented why
- There's no better alternative

## Continuous Monitoring

### Daily Checks
```bash
# Quick check before starting work
cargo check

# Before committing
cargo fmt && cargo clippy -- -D warnings && cargo test
```

### Weekly Checks
```bash
# Update dependencies
cargo update

# Check for outdated dependencies
cargo install cargo-outdated
cargo outdated

# Security audit
cargo audit
```

### Monthly Checks
```bash
# Update Rust toolchain
rustup update

# Check for breaking changes
cargo check --all-targets
cargo test --all-targets
```

## Zero-Warning Policy

**All code MUST have zero warnings.**

### Why?
- Warnings hide real issues
- Warnings accumulate over time
- Warnings become noise
- Warnings indicate code smells

### How?
1. Fix warnings immediately
2. Treat warnings as errors in CI
3. Review clippy suggestions
4. Refactor problematic patterns
5. Document suppressions

## Checklist

Before every commit:
- [ ] `cargo fmt` - Code is formatted
- [ ] `cargo clippy -- -D warnings` - No clippy warnings
- [ ] `cargo test` - All tests pass
- [ ] `cargo check` - Code compiles
- [ ] `cargo audit` - No security vulnerabilities
- [ ] All new code has tests
- [ ] All public APIs have documentation
- [ ] No `unwrap()` or `expect()` in production code
- [ ] No `todo!()` or `unimplemented!()` in production code

## Remember

- **Linting is not optional**
- **Fix warnings immediately**
- **Automate checks in CI/CD**
- **Configure editor for format-on-save**
- **Zero warnings is the only acceptable state**
- **Clippy suggestions improve code quality**
- **Security audits prevent vulnerabilities**
