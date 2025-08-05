use std::fmt::Display;

#[cfg(doc)]
use crate::ErrorLevel;

#[derive(PartialEq, Eq, Debug, Copy, Clone, Default)]
/// The strictness to operate in, this defines at which [`ErrorLevel`] the program should stop execution upon finding an error.
pub enum StrictnessLevel {
    /// With `Strict` the program will always stop execution upon finding an error.
    Strict,
    /// With `Medium` the program will allow [`ErrorLevel::GeneralWarning`].
    #[default]
    Medium,
    /// With `Loose` the program will allow [`ErrorLevel::GeneralWarning`] and [`ErrorLevel::LooseWarning`].
    Loose,
}

impl Display for StrictnessLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Strict => "Strict",
                Self::Medium => "Medium",
                Self::Loose => "Loose",
            }
        )
    }
}
