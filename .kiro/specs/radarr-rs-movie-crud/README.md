# Radarr-RS Movie CRUD Specification

## Overview

This specification defines the Rust port of Radarr's movie CRUD operations, maintaining full API compatibility with the existing Radarr V3 API while providing equivalent functionality for managing movie entities.

**Project Structure**: The Rust implementation is being built alongside the existing C# codebase:
- **Rust code**: Root directory (`radarr_core/`, `radarr_api/`, `Cargo.toml`)
- **Legacy C# code**: `legacy/` directory (for reference during conversion)

## Specification Documents

- **[requirements.md](./requirements.md)** - 10 requirements with EARS/INCOSE compliance
- **[design.md](./design.md)** - Architecture, components, and data models
- **[tasks.md](./tasks.md)** - Implementation plan with 10 major tasks and 50+ subtasks

## Steering Documents

All code must follow these guidelines (automatically included in Kiro context):

- **[rust-best-practices.md](../../steering/rust-best-practices.md)** - Rust coding standards
- **[solid-principles.md](../../steering/solid-principles.md)** - SOLID principles in Rust
- **[dry-principle.md](../../steering/dry-principle.md)** - Don't Repeat Yourself
- **[lint-and-fix.md](../../steering/lint-and-fix.md)** - Mandatory linting and formatting
- **[no-feature-left-behind.md](../../steering/no-feature-left-behind.md)** - Complete implementation requirements
- **[git-workflow.md](../../steering/git-workflow.md)** - Branch-per-task workflow

## Getting Started

### 1. Review the Specification

Read through the requirements and design documents to understand the scope and architecture.

### 2. Set Up Development Branch

```bash
# Create development branch if it doesn't exist
git checkout -b develop

# Push to remote
git push -u origin develop
```

### 3. Start First Task

```bash
# Create feature branch for Task 1
git checkout develop
git checkout -b feature/task-1-setup-project-structure

# Open tasks.md in Kiro and click "Start task" next to Task 1
```

### 4. Follow the Workflow

For each task:
1. Create feature branch from develop
2. Implement ALL acceptance criteria
3. Write ALL tests
4. Add ALL documentation
5. Run quality checks (fmt, clippy, test, audit)
6. Merge to develop
7. Delete feature branch
8. Start next task

## Quality Standards

### Before Every Merge

```bash
# Format code
cargo fmt

# Fix all clippy warnings
cargo clippy -- -D warnings

# Run all tests
cargo test

# Security audit
cargo audit
```

### Definition of Done

A task is complete when:
- ✅ All acceptance criteria met
- ✅ All tests written and passing
- ✅ All documentation complete
- ✅ Zero clippy warnings
- ✅ Code formatted
- ✅ No TODOs in code
- ✅ Merged to develop

## Architecture Overview

```
radarr_core/
├── domain/          # Domain models (Movie, MovieMetadata, types)
├── repository/      # Data access layer (traits + implementations)
├── service/         # Business logic layer
└── error.rs         # Domain errors

radarr_api/
├── handlers/        # HTTP request handlers
├── resources/       # API DTOs
├── validation/      # Request validation
└── error.rs         # API errors
```

## Technology Stack

- **Web Framework**: Axum
- **Database**: sqlx (SQLite + PostgreSQL)
- **Serialization**: serde + serde_json
- **Validation**: validator
- **Error Handling**: thiserror + anyhow
- **Async Runtime**: tokio

## Task List Summary

1. **Set up project structure** - Workspace, dependencies, migrations
2. **Implement domain models** - Movie, MovieMetadata, types, enums
3. **Create database schema** - Migrations for SQLite and PostgreSQL
4. **Implement repository layer** - Traits and implementations
5. **Implement service layer** - Business logic
6. **Implement API resources** - DTOs and conversions
7. **Implement HTTP handlers** - REST endpoints
8. **Implement error handling** - Error types and responses
9. **Configure routing** - Application setup
10. **Write integration tests** - End-to-end testing

## Success Criteria

- ✅ Pass all ported unit tests
- ✅ API responses match original Radarr
- ✅ Existing frontend works unchanged
- ✅ 5x+ performance improvement
- ✅ <100MB memory footprint
- ✅ Zero clippy warnings
- ✅ >80% test coverage

## Development Principles

### SOLID
- Single Responsibility: Each module has one purpose
- Open/Closed: Extend via traits, not modification
- Liskov Substitution: All implementations honor contracts
- Interface Segregation: Small, focused traits
- Dependency Inversion: Depend on abstractions

### DRY
- Extract common logic into reusable functions
- Single source of truth for data structures
- Avoid duplicating queries and error handling

### No Feature Left Behind
- Complete ALL acceptance criteria
- Write ALL tests
- Add ALL documentation
- Handle ALL error cases
- No TODOs in production code

### Always Lint and Fix
- Zero warnings policy
- Format on save
- Fix clippy issues immediately
- Run checks before every commit

### Git Workflow
- One branch per task
- Always branch from develop
- Merge before next task
- Keep branches short-lived
- Clean commit history

## Next Steps

1. Open `.kiro/specs/radarr-rs-movie-crud/tasks.md`
2. Click "Start task" next to Task 1
3. Follow the git workflow to create feature branch
4. Implement the task following all steering guidelines
5. Merge and move to next task

## Questions?

Refer to the steering documents for detailed guidance on:
- Rust best practices
- SOLID principles
- DRY principle
- Linting and formatting
- Complete implementation
- Git workflow

Happy coding! 🦀
