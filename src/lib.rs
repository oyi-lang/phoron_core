//! `phoron_core` is a library to enable serialization and deserialization of JVM bytecode.
//! It consists of two main modules:
//!  - deserializer : read in the raw bytes of a JVM `class` file and construct an object model.
//!  - serializer : take the object model representation and construct the JVM `class` file bytes
//!  from it.
pub mod deserializer;
pub mod error;
pub mod model;
pub mod rw;
pub mod serializer;
