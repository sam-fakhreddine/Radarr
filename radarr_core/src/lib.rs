//! Radarr Core Library
//!
//! This crate provides the core domain models, business logic, and data access
//! layer for the Radarr Rust implementation.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

/// Domain models and types
pub mod domain;

/// Repository traits and implementations
pub mod repository;

/// Service layer with business logic
pub mod service;

/// Error types
pub mod error;

/// Validation helpers
pub mod validation;
