#[cfg(doc)]
use crate::ErrorLevel;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
/// The strictness to operate in, this defines at which [`ErrorLevel`] the program should stop execution upon finding an error.
pub enum StrictnessLevel {
    /// With `Strict` the program will always stop execution upon finding an error.
    Strict,
    /// With `Medium` the program will allow [`ErrorLevel::GeneralWarning`].
    Medium,
    /// With `Loose` the program will allow [`ErrorLevel::GeneralWarning`] and [`ErrorLevel::LooseWarning`].
    Loose,
}
