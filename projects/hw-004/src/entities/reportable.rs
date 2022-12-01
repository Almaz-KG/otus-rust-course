use std::fmt::Display;

/// A trait for making possible some entity to report.
/// Any entity, which want to provide internal status must
/// implement this trait as well as [Display] trait.
/// Display trait must be implemented in order to be able
/// provide the full description of the entity. Whereas,
/// the [Reportable] trait devoted for the short, fast as small
/// current status reporting
pub trait Reportable: Display {
    /// Main function for reporting. For the first implementation,
    /// it returns [String], but in the future implementation
    /// it may be refactored to return some lean and complex type
    fn report(&self) -> String;
}