---
inclusion: always
---

# Rust Best Practices

## Code Quality Standards

### Error Handling

- **ALWAYS** use `Result<T, E>` for fallible operations
- **NEVER** use `.unwrap()` or `.expect()` in production code
- Use `thiserror` for domain errors with descriptive messages
- Use `anyhow` only for application-level error aggregation
- Propagate errors with `?` operator
- Log errors at appropriate levels before returning

### Type Safety

- Leverage Rust's type system for compile-time guarantees
- Use newtypes for domain concepts (e.g., `MovieId(i32)` instead of raw `i32`)
- Prefer `Option<T>` over nullable patterns
- Use enums for state machines and variants
- Implement `From`/`TryFrom` for type conversions

### Ownership and Borrowing

- Prefer borrowing (`&T`) over cloning when possible
- Use `Arc<T>` for shared ownership across threads
- Use `Cow<T>` for clone-on-write scenarios
- Avoid `Rc<RefCell<T>>` in async code
- Document lifetime requirements clearly

### Async/Await

- Use `async fn` for I/O-bound operations
- Use `tokio::spawn` for concurrent tasks
- Use `tokio::spawn_blocking` for CPU-intensive work
- Always set timeouts for external operations
- Use `select!` for cancellation patterns
- Avoid blocking operations in async contexts

### Memory Management

- Minimize allocations in hot paths
- Use `Vec::with_capacity` when size is known
- Prefer stack allocation over heap when possible
- Use `Box<T>` for large stack objects
- Profile memory usage with tools like `heaptrack`

### Traits and Generics

- Define traits for abstraction boundaries
- Use trait objects (`dyn Trait`) for runtime polymorphism
- Use generics for compile-time polymorphism
- Implement standard traits (`Debug`, `Clone`, `Default`) where appropriate
- Use `#[async_trait]` for async trait methods

### Testing

- Write unit tests for all business logic
- Write integration tests for API endpoints
- Use `#[cfg(test)]` for test-only code
- Mock external dependencies with traits
- Test error paths, not just happy paths
- Use property-based testing for complex logic

### Documentation

- Document all public APIs with `///` doc comments
- Include examples in doc comments
- Document panics, errors, and safety requirements
- Keep README.md up to date
- Document architectural decisions

### Performance

- Profile before optimizing
- Use `#[inline]` judiciously
- Avoid premature optimization
- Use iterators over explicit loops
- Leverage zero-cost abstractions
- Benchmark critical paths

## Code Organization

### Module Structure

```rust
// Good: Clear module hierarchy
mod domain {
    pub mod movie;
    pub mod types;
}

mod repository {
    pub mod movie_repository;
    pub mod traits;
}

// Bad: Flat structure
mod movie;
mod movie_repository;
mod movie_service;
```

### Visibility

- Make fields private by default
- Expose only necessary APIs as `pub`
- Use `pub(crate)` for internal APIs
- Use `pub(super)` for parent module access

### Naming Conventions

- Use `snake_case` for functions, variables, modules
- Use `PascalCase` for types, traits, enums
- Use `SCREAMING_SNAKE_CASE` for constants
- Prefix boolean functions with `is_`, `has_`, `can_`
- Use descriptive names, avoid abbreviations

## Dependency Management

### Cargo.toml

- Pin major versions for stability
- Use workspace dependencies for consistency
- Minimize dependency count
- Audit dependencies regularly with `cargo audit`
- Document why each dependency is needed

### Feature Flags

- Use features for optional functionality
- Keep default features minimal
- Document feature combinations

## Security

### Input Validation

- Validate all external input
- Sanitize data before database operations
- Use parameterized queries (sqlx handles this)
- Validate file paths to prevent traversal
- Rate limit API endpoints

### Secrets Management

- Never hardcode secrets
- Use environment variables or secret managers
- Don't log sensitive data
- Clear sensitive data from memory when done

## Rust-Specific Patterns

### Builder Pattern

```rust
// Use for complex object construction
let movie = Movie::builder()
    .title("Inception")
    .year(2010)
    .monitored(true)
    .build()?;
```

### Newtype Pattern

```rust
// Use for type safety
pub struct MovieId(i32);
pub struct TmdbId(i32);
```

### Type State Pattern

```rust
// Use for compile-time state validation
struct Movie<State> {
    data: MovieData,
    _state: PhantomData<State>,
}

struct Draft;
struct Published;

impl Movie<Draft> {
    fn publish(self) -> Movie<Published> { ... }
}
```

## Linting and Formatting

### Required Tools

- `rustfmt` - Code formatting (run on save)
- `clippy` - Linting (fix all warnings)
- `cargo check` - Fast compilation check
- `cargo test` - Run all tests
- `cargo audit` - Security audits

### Clippy Configuration

```toml
# Cargo.toml
[lints.clippy]
all = "warn"
pedantic = "warn"
nursery = "warn"
unwrap_used = "deny"
expect_used = "deny"
panic = "deny"
```

### Pre-commit Checks

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo audit
```

## Common Pitfalls to Avoid

### Don't

- ❌ Use `.unwrap()` in production
- ❌ Ignore compiler warnings
- ❌ Use `unsafe` without documentation
- ❌ Block in async functions
- ❌ Clone unnecessarily
- ❌ Use `String` when `&str` suffices
- ❌ Ignore error results
- ❌ Use global mutable state

### Do

- ✅ Handle all errors explicitly
- ✅ Fix all clippy warnings
- ✅ Write tests for new code
- ✅ Document public APIs
- ✅ Use type system for correctness
- ✅ Profile before optimizing
- ✅ Review code before committing
- ✅ Keep dependencies minimal
