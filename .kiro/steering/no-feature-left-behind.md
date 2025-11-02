---
inclusion: always
---

# No Feature Left Behind

**Every feature must be fully implemented, tested, and documented before moving to the next.**

## Core Principle

When implementing a feature:
1. ✅ Complete ALL acceptance criteria
2. ✅ Write ALL tests (unit + integration)
3. ✅ Add ALL documentation
4. ✅ Handle ALL error cases
5. ✅ Verify ALL edge cases

**Never leave a feature partially implemented.**

## Definition of Done

A feature is DONE when:

### 1. Implementation Complete
- [ ] All acceptance criteria from requirements are met
- [ ] All functions/methods are implemented
- [ ] No `todo!()`, `unimplemented!()`, or placeholder code
- [ ] All error paths are handled
- [ ] All edge cases are covered

### 2. Tests Written
- [ ] Unit tests for all business logic
- [ ] Integration tests for API endpoints
- [ ] Error case tests
- [ ] Edge case tests
- [ ] All tests pass
- [ ] Test coverage > 80%

### 3. Documentation Added
- [ ] Public APIs have doc comments
- [ ] Complex logic is explained
- [ ] Examples provided where helpful
- [ ] README updated if needed
- [ ] Architecture docs updated if needed

### 4. Code Quality
- [ ] `cargo fmt` passes
- [ ] `cargo clippy -- -D warnings` passes
- [ ] No compiler warnings
- [ ] Code reviewed (if applicable)
- [ ] Follows SOLID principles
- [ ] Follows DRY principle
- [ ] Follows Rust best practices

### 5. Verification
- [ ] Manual testing completed
- [ ] Integration with existing features verified
- [ ] Performance acceptable
- [ ] No regressions introduced

## Implementation Workflow

### Step 1: Read Requirements
```
Before writing ANY code:
1. Read the requirement completely
2. Understand all acceptance criteria
3. Identify dependencies
4. Plan the implementation
5. Identify test cases
```

### Step 2: Implement Core Logic
```rust
// ✅ Good: Complete implementation
pub async fn add_movie(&self, movie: Movie) -> Result<Movie> {
    // Validate movie doesn't exist
    if let Some(_) = self.find_by_tmdb_id(movie.tmdb_id).await? {
        return Err(Error::Validation(
            "Movie already exists".to_string()
        ));
    }
    
    // Validate quality profile exists
    if !self.quality_profile_exists(movie.quality_profile_id).await? {
        return Err(Error::Validation(
            "Quality profile does not exist".to_string()
        ));
    }
    
    // Set added timestamp
    let mut movie = movie;
    movie.added = Utc::now();
    
    // Insert movie
    let created = self.repository.insert(&movie).await?;
    
    // Publish event
    self.events.publish(MovieAddedEvent { movie: created.clone() }).await?;
    
    Ok(created)
}

// ❌ Bad: Incomplete implementation
pub async fn add_movie(&self, movie: Movie) -> Result<Movie> {
    // TODO: Add validation
    self.repository.insert(&movie).await
}
```

### Step 3: Write Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    // ✅ Test happy path
    #[tokio::test]
    async fn test_add_movie_success() {
        let service = setup_test_service().await;
        let movie = create_test_movie();
        
        let result = service.add_movie(movie).await;
        
        assert!(result.is_ok());
        let created = result.unwrap();
        assert!(created.id > 0);
        assert_eq!(created.title, "Test Movie");
    }
    
    // ✅ Test error cases
    #[tokio::test]
    async fn test_add_movie_duplicate() {
        let service = setup_test_service().await;
        let movie = create_test_movie();
        
        service.add_movie(movie.clone()).await.unwrap();
        let result = service.add_movie(movie).await;
        
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::Validation(_)));
    }
    
    #[tokio::test]
    async fn test_add_movie_invalid_quality_profile() {
        let service = setup_test_service().await;
        let mut movie = create_test_movie();
        movie.quality_profile_id = 999; // Non-existent
        
        let result = service.add_movie(movie).await;
        
        assert!(result.is_err());
    }
    
    // ✅ Test edge cases
    #[tokio::test]
    async fn test_add_movie_empty_path() {
        let service = setup_test_service().await;
        let mut movie = create_test_movie();
        movie.path = String::new();
        movie.root_folder_path = None;
        
        let result = service.add_movie(movie).await;
        
        assert!(result.is_err());
    }
}
```

### Step 4: Add Documentation
```rust
/// Adds a new movie to the library.
///
/// # Arguments
///
/// * `movie` - The movie to add
///
/// # Returns
///
/// Returns the created movie with generated ID and timestamp.
///
/// # Errors
///
/// Returns an error if:
/// - Movie already exists (by TMDB ID)
/// - Quality profile doesn't exist
/// - Path validation fails
/// - Database operation fails
///
/// # Examples
///
/// ```
/// let movie = Movie {
///     title: "Inception".to_string(),
///     year: 2010,
///     tmdb_id: 27205,
///     quality_profile_id: 1,
///     path: "/movies/Inception (2010)".to_string(),
///     monitored: true,
///     ..Default::default()
/// };
///
/// let created = service.add_movie(movie).await?;
/// assert!(created.id > 0);
/// ```
pub async fn add_movie(&self, movie: Movie) -> Result<Movie> {
    // Implementation
}
```

### Step 5: Verify Complete
```bash
# Run all checks
cargo fmt
cargo clippy -- -D warnings
cargo test
cargo doc --no-deps --open

# Manual verification
# - Test in running application
# - Verify API responses
# - Check database state
# - Test error scenarios
```

## Anti-Patterns to Avoid

### ❌ Partial Implementation
```rust
// DON'T DO THIS
pub async fn add_movie(&self, movie: Movie) -> Result<Movie> {
    // TODO: Add validation later
    self.repository.insert(&movie).await
}
```

### ❌ Skipping Tests
```rust
// DON'T DO THIS
// "I'll write tests later"
pub async fn add_movie(&self, movie: Movie) -> Result<Movie> {
    // Full implementation but no tests
}
```

### ❌ Incomplete Error Handling
```rust
// DON'T DO THIS
pub async fn add_movie(&self, movie: Movie) -> Result<Movie> {
    // Only handles happy path
    self.repository.insert(&movie).await
    // What about duplicates? Invalid data? Database errors?
}
```

### ❌ Missing Documentation
```rust
// DON'T DO THIS
pub async fn add_movie(&self, movie: Movie) -> Result<Movie> {
    // No documentation
    // What does this do? What errors can it return?
}
```

### ❌ Leaving TODOs
```rust
// DON'T DO THIS
pub async fn add_movie(&self, movie: Movie) -> Result<Movie> {
    // TODO: Validate quality profile
    // TODO: Check for duplicates
    // TODO: Publish event
    self.repository.insert(&movie).await
}
```

## Feature Checklist Template

For each feature, create a checklist:

```markdown
## Feature: Add Movie

### Requirements
- [ ] Requirement 2.1: Create movie record
- [ ] Requirement 2.2: Validate path or root folder
- [ ] Requirement 2.3: Construct path from root folder
- [ ] Requirement 2.4: Validate quality profile
- [ ] Requirement 2.5: Set added timestamp
- [ ] Requirement 2.6: Return 201 status
- [ ] Requirement 2.7: Check for duplicates
- [ ] Requirement 2.8: Return validation errors

### Implementation
- [ ] Service method implemented
- [ ] Repository method implemented
- [ ] API handler implemented
- [ ] Request/response DTOs created
- [ ] Validation logic added
- [ ] Error handling complete

### Tests
- [ ] Unit test: successful creation
- [ ] Unit test: duplicate movie
- [ ] Unit test: invalid quality profile
- [ ] Unit test: missing path
- [ ] Integration test: API endpoint
- [ ] Integration test: database persistence
- [ ] Integration test: error responses

### Documentation
- [ ] Service method documented
- [ ] API endpoint documented
- [ ] Error cases documented
- [ ] Examples provided

### Verification
- [ ] Manual testing complete
- [ ] All tests pass
- [ ] Clippy passes
- [ ] Format check passes
- [ ] Code reviewed
```

## Progress Tracking

### Daily Standup Questions
1. What feature did I complete yesterday?
2. Is it FULLY complete (implementation + tests + docs)?
3. What feature am I working on today?
4. What's blocking me from completing it?

### Weekly Review
1. How many features were completed this week?
2. Are there any partially implemented features?
3. What's the test coverage?
4. Are there any TODOs in the codebase?

### Monthly Audit
```bash
# Find incomplete features
rg "TODO|FIXME|XXX|HACK" --type rust

# Check test coverage
cargo tarpaulin --out Html

# Find undocumented public APIs
cargo rustdoc -- -D missing_docs
```

## When to Move On

Move to the next feature ONLY when:

1. ✅ All acceptance criteria are met
2. ✅ All tests are written and passing
3. ✅ All documentation is complete
4. ✅ All clippy warnings are fixed
5. ✅ Code is formatted
6. ✅ Manual testing is done
7. ✅ Code is reviewed (if applicable)
8. ✅ No TODOs remain
9. ✅ Feature is merged to main branch

## Exception Handling

If you MUST leave something incomplete:

1. **Document it clearly**
   ```rust
   // INCOMPLETE: Missing validation for edge case X
   // Tracked in issue #123
   // Will be completed before release
   ```

2. **Create a tracking issue**
   - What's incomplete
   - Why it's incomplete
   - When it will be completed
   - Who's responsible

3. **Mark it in code**
   ```rust
   #[allow(clippy::todo)]
   todo!("Implement validation - Issue #123")
   ```

4. **Add to technical debt log**
   - Keep a TECHNICAL_DEBT.md file
   - List all incomplete features
   - Review regularly

## Remember

- **Quality over speed**
- **Complete features, not partial implementations**
- **Tests are not optional**
- **Documentation is not optional**
- **TODOs are technical debt**
- **Partial features create bugs**
- **Finish what you start**

## Enforcement

### Code Review Checklist
- [ ] Feature is complete per requirements
- [ ] All tests are present and passing
- [ ] Documentation is complete
- [ ] No TODOs or FIXMEs
- [ ] All error cases handled
- [ ] Edge cases covered

### CI/CD Checks
```yaml
# Fail build if TODOs exist
- name: Check for TODOs
  run: |
    if rg "TODO|FIXME" --type rust src/; then
      echo "Found TODOs in code"
      exit 1
    fi

# Fail build if test coverage < 80%
- name: Check coverage
  run: |
    cargo tarpaulin --out Xml
    if [ $(grep -oP 'line-rate="\K[^"]+' cobertura.xml | awk '{if ($1 < 0.8) exit 1}') ]; then
      echo "Test coverage below 80%"
      exit 1
    fi
```

## Success Metrics

- ✅ Zero TODOs in production code
- ✅ Test coverage > 80%
- ✅ All public APIs documented
- ✅ Zero clippy warnings
- ✅ All features fully implemented
- ✅ No partial implementations
- ✅ Technical debt log is empty
