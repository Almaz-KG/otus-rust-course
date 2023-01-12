//! This is a container for different variety of entities for the OTUS smart-home project.
//!
//! Smart home project is a series of small rust logics aiming to provide the complete smart
//! house, with variety smart controllers and devices.
//!
//! The `entities` sub module contains as name claims the entities for the project use case.
//!
//! Hopefully, in the future home works, a bunch of new submodules will be included here, so this
//! module will not be so lonely

/// The list of more or less plain Rust entities involved to the project.
pub mod entities;

/// A CLI module encapsulates structures and logics behind the command line interface. This
/// module contains logics for interacting with user via CLI. Who knows, maybe we will do some
/// Graphical User interface, so we will have a better option, instead of boring CLI
pub mod cli;

/// A module which contains a structs and trait for the TCP and other Servers. Right now, only
/// Tcp Server is implemented, but in the future home-works in might be adjusted with other
/// servers as well.
pub mod server;
