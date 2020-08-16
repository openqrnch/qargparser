mod err;
mod parser;
mod prsrutil;
mod spec;

pub use crate::parser::Parser;
pub use crate::spec::{Builder, Nargs, Spec};

pub use crate::err::ErrKind;

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
