//! An entities module in the Otus Smart Home project
//!
//! This module contains several submodules for their intentional use case. The name of the
//! submodule should give you the full picture of the purpose of the module.

/// A [house] submodule contains entities belong to house entity, such as `Home`, `Room`, etc.
/// Each entity should contain its own builder, so the usage of the entity should be super easy
/// and obvious for end user
pub mod house;

/// A [devices] submodule contains entities related to smart devices, such as `Socket`,
/// `Thermometer`, and others. These entities also have their own builders for simplicity of usage.
pub mod devices;

/// A [reportable] submodule contains an useful trait for displaying and reporting functions
mod reportable;
pub use reportable::{ReportError, Reportable};

/// A [measure] submodule holds a public trait [CanMeasure](measure/CanMeasure) which is an
/// interface for any object which can make some measurement of the surrounding environment. This
/// is relatively simple interface object, which will allow to store group of devices in single
/// container. All details of measurement should be hidden in the exact device implementation.
mod measure;
pub use measure::{CanMeasure, MeasureError};
