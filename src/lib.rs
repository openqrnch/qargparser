//! This command line parser is based on a C++ command line parser which is
//! based on a C command line parser which in turn was inspired by the Python
//! argparse module.
//!
//! The basic use pattern for qargparser is:
//! 1. Create a context struct for storing parser output.
//! 2. Create one or more [`Builder`].
//! 3. Create [`Spec`] objects from the [`Builder`] objects
//!    by calling [`Builder::build()`](Builder::build), passing a function for
//!    processing the option/argument.
//! 4. Create a [`Parser`] object ([`Parser::from_env()`](Parser::from_env)
//!    or [`Parser::from_args()`](Parser::from_args)) handing over the
//!    ownership of a parser context struct.
//! 5. Add the [`Spec`] objects to the parser object using
//!    [`Parser::add()`](Parser::add).
//! 6. Call the parser.
//! 7. At this point, if successful, the parser context should be populated
//!    with data from the command line.  Call
//!    [`Parser::into_ctx()`](Parser::into_ctx) to regain ownership of the
//!    context.
//!
//! One of the minor goals of qargparser is to simplify reuse of
//! option/argument specs between parsers.
//!
//! # ToDo
//! - Currently converts argument strings to UTF-8.  Should support
//!   `OsStr(ing)`.

mod err;
mod parser;
mod prsrutil;
mod spec;

pub use crate::parser::Parser;
pub use crate::spec::{Builder, Nargs, Spec};

pub use crate::err::ErrKind;

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
