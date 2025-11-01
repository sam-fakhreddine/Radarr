---
inclusion: always
---

# Git Workflow - Branch Per Task

**Every task MUST be developed in its own feature branch and merged to rust before starting the next task.**

## Core Principles

1. **One branch per task** - Each task gets its own feature branch
2. **Always branch from rust** - Start fresh from the latest rust branch state
3. **Merge before moving on** - Complete and merge task before starting next
4. **Keep branches short-lived** - Merge within 1-2 days maximum
5. **Clean history** - Squash commits when merging

## Branch Naming Convention

```bash
# Format: feature/task-{number}-{short-description}
feature/task-1-setup-project-structure
feature/task-2.1-create-movie-status-enum
feature/task-2.2-create-ratings-struct
feature/task-3.1-create-movies-table-migration
```

### Rules
- Use `feature/` prefix for new features
- Use `fix/` prefix for bug fixes
- Include task number from tasks.md
- Use kebab-case for description
- Keep description short but descriptive
- Use subtask numbers (e.g., 2.1, 2.2) when applicable

## Workflow Steps

### 1. Before Starting a Task

```bash
# Ensure you're on rust branch
git checkout rust

# Pull latest changes
git pull origin rust

# Verify clean working directory
git status

# Create feature branch for the task
git checkout -b feature/task-1-setup-project-structure
```

### 2. During Task Development

```bash
# Make changes and commit frequently
git add .
git commit -m "Add Cargo workspace configuration"

git add .
git commit -m "Add axum and sqlx dependencies"

git add .
git commit -m "Configure clippy lints"

# Push to remote regularly (backup and collaboration)
git push origin feature/task-1-setup-project-structure
```

### 3. Before Completing Task

```bash
# Run all quality checks
cargo fmt
cargo clippy -- -D warnings
cargo test
cargo audit

# Ensure all changes are committed
git status

# Update from rust (in case of changes)
git checkout rust
git pull origin rust
git checkout feature/task-1-setup-project-structure
git rebase rust

# Resolve any conflicts if they occur
```

### 4. Merging Task

```bash
# Switch to rust branch
git checkout rust

# Merge feature branch (use --squash for clean history)
git merge --squash feature/task-1-setup-project-structure

# Create a single commit for the task
git commit -m "Complete Task 1: Set up project structure and dependencies

- Created Cargo workspace with radarr_core and radarr_api crates
- Added dependencies: axum, sqlx, serde, tokio, thiserror, validator
- Configured sqlx for compile-time query checking
- Set up database migration directory structure
- Configured clippy lints for strict checking

Requirements: 5.1, 5.2, 5.3"

# Push to remote
git push origin rust

# Delete feature branch locally
git branch -d feature/task-1-setup-project-structure

# Delete feature branch remotely
git push origin --delete feature/task-1-setup-project-structure
```

### 5. Starting Next Task

```bash
# Already on rust with latest changes
git checkout rust
git pull origin rust

# Create new branch for next task
git checkout -b feature/task-2.1-create-movie-status-enum
```

## Commit Message Guidelines

### Format
```
<type>: <subject>

<body>

<footer>
```

### Types
- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation changes
- `test:` - Adding or updating tests
- `refactor:` - Code refactoring
- `style:` - Formatting changes
- `chore:` - Maintenance tasks

### Examples

```bash
# Good commit messages
git commit -m "feat: implement MovieStatusType enum with serde support"
git commit -m "test: add unit tests for is_available method"
git commit -m "fix: handle missing release dates in availability calculation"
git commit -m "docs: add doc comments to MovieRepository trait"

# Bad commit messages
git commit -m "wip"
git commit -m "fix stuff"
git commit -m "updates"
```

### Task Completion Commit
```bash
git commit -m "Complete Task 2.1: Create MovieStatusType enum

- Define TBA, Announced, InCinemas, Released variants
- Implement serde serialization with camelCase
- Implement sqlx Type trait for database mapping
- Add unit tests for serialization
- Add documentation

Requirements: 1.4, 8.5"
```

## Branch Protection Rules

### Rust Branch
- Require all tests to pass
- Require clippy to pass with zero warnings
- Require code formatting check
- No direct commits (only merges from feature branches)

### Feature Branches
- Must be up-to-date with rust before merging
- Must pass all CI checks
- Must have task completion commit message

## Handling Conflicts

### If Conflicts Occur During Rebase
```bash
# View conflicts
git status

# Edit conflicting files
# Look for <<<<<<< HEAD markers

# After resolving conflicts
git add .
git rebase --continue

# If you want to abort
git rebase --abort
```

### Prevention
- Keep feature branches short-lived
- Merge to develop frequently
- Pull from develop regularly
- Communicate with team about overlapping work

## Emergency Fixes

### Hotfix Workflow
```bash
# Create hotfix branch from rust
git checkout rust
git checkout -b hotfix/critical-bug-description

# Make fix
# ... edit files ...

# Commit
git commit -m "hotfix: fix critical bug in movie validation"

# Merge back immediately
git checkout rust
git merge --no-ff hotfix/critical-bug-description
git push origin rust

# Delete hotfix branch
git branch -d hotfix/critical-bug-description
```

## Task Checklist Template

Before merging each task:

```markdown
## Task Completion Checklist

### Code Quality
- [ ] All code formatted (`cargo fmt`)
- [ ] All clippy warnings fixed (`cargo clippy -- -D warnings`)
- [ ] All tests pass (`cargo test`)
- [ ] No security vulnerabilities (`cargo audit`)
- [ ] Code reviewed (if applicable)

### Documentation
- [ ] Public APIs documented
- [ ] Complex logic explained
- [ ] Examples provided where needed

### Git
- [ ] All changes committed
- [ ] Rebased on latest rust
- [ ] No merge conflicts
- [ ] Feature branch pushed to remote
- [ ] Commit message follows guidelines

### Task Requirements
- [ ] All acceptance criteria met
- [ ] All subtasks completed
- [ ] Requirements referenced in commit message
- [ ] Task marked as complete in tasks.md
```

## CI/CD Integration

### GitHub Actions Example
```yaml
name: Feature Branch CI

on:
  push:
    branches:
      - 'feature/**'
      - 'fix/**'

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

## Common Mistakes to Avoid

### ❌ Don't
- Don't work directly on rust branch
- Don't merge incomplete tasks
- Don't skip quality checks before merging
- Don't leave feature branches unmerged for days
- Don't create branches from other feature branches
- Don't force push to rust
- Don't merge without updating from rust first

### ✅ Do
- Create a new branch for each task
- Merge completed tasks promptly
- Run all checks before merging
- Keep feature branches short-lived
- Always branch from rust
- Use descriptive branch names
- Write clear commit messages
- Delete merged branches

## Workflow Diagram

```
rust (main development branch)
   │
   ├─── feature/task-1-setup-project
   │    │
   │    ├─ commit: Add workspace
   │    ├─ commit: Add dependencies
   │    └─ commit: Configure lints
   │    │
   │    └─── [MERGE] Complete Task 1
   │
   ├─── feature/task-2.1-create-enum
   │    │
   │    ├─ commit: Define enum
   │    ├─ commit: Add serde
   │    └─ commit: Add tests
   │    │
   │    └─── [MERGE] Complete Task 2.1
   │
   ├─── feature/task-2.2-create-ratings
   │    │
   │    └─── [MERGE] Complete Task 2.2
   │
   └─── ... continue for each task
```

## Task Tracking

### Update tasks.md After Merge
```bash
# After merging task, update tasks.md
# Change [ ] to [x] for completed task

- [x] 1. Set up Rust project structure and dependencies
  - Created workspace with radarr_core and radarr_api crates
  - Added dependencies: axum, sqlx, serde, tokio, thiserror, validator
  - Configured sqlx for compile-time query checking
  - Set up database migration directory structure
  - _Requirements: 5.1, 5.2, 5.3_
```

## Remember

- **One task, one branch, one merge**
- **Always start from develop**
- **Merge before moving to next task**
- **Keep branches short-lived**
- **Run all checks before merging**
- **Write descriptive commit messages**
- **Delete merged branches**
- **Update tasks.md after merge**

## Quick Reference

```bash
# Start new task
git checkout rust && git pull origin rust
git checkout -b feature/task-X-description

# During development
git add . && git commit -m "descriptive message"
git push origin feature/task-X-description

# Before merging
cargo fmt && cargo clippy -- -D warnings && cargo test
git checkout rust && git pull origin rust
git checkout feature/task-X-description && git rebase rust

# Merge task
git checkout rust
git merge --squash feature/task-X-description
git commit -m "Complete Task X: description\n\n- bullet points\n\nRequirements: X.X"
git push origin rust
git branch -d feature/task-X-description
git push origin --delete feature/task-X-description
```
