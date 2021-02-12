/// The context of an error
mod context;
/// The severity of an error
mod errorlevel;
/// An error with all its properties
mod pdberror;

pub use context::{Context, Position};
pub use errorlevel::ErrorLevel;
pub use pdberror::PDBError;
