# Contributing to Oculus

Thank you for your interest in contributing! To ensure the log analyzer remains fast, reliable, and easy to maintain, we ask that all contributors follow these coding rules.

## The "Golden Rule"

Before submitting a Pull Request, you **must** run these three commands locally. Your PR will not be merged if any of these fail in CI.

```bash
# 1. Format your code
cargo fmt

# 2. Run the linter
cargo clippy --all-targets --all-features -- -D warnings

# 3. Pass all tests
cargo test --all-features
```

## Why these rules matter

### 1. `cargo fmt`

Rust has a standard code formatting style enforced by `rustfmt`. Running `cargo fmt` ensures that all code looks the same, eliminating arguments over tabs vs. spaces or where curly braces should go. It makes the codebase much easier for everyone to read.

### 2. `cargo clippy`

Clippy is Rust's official linter. It catches common mistakes, unidiomatic code (code that "doesn't look like Rust"), and potential performance issues. By adding `-D warnings`, we tell Clippy to treat all warnings as errors—meaning we don't tolerate sloppy code!

### 3. `cargo test`

Logs are complex, and our parsers need to be rock solid. You must ensure all existing unit and integration tests pass. If you are adding a new parser or feature, please include new tests to prove it works.
