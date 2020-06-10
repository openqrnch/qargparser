use std::fmt;
use std::error::Error as StdError;
use std::rc::Rc;
use std::cell::RefCell;

use crate::spec::Spec;

#[derive(Clone)]
pub struct SpecErr<C> {
  pub spec: Rc<RefCell<Spec<C>>>,
  pub msg: String
}

impl<C> fmt::Display for SpecErr<C> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_fmt(format_args!("spec"))
  }
}


#[derive(Clone)]
pub enum ErrKind<C> {
  MissArg(SpecErr<C>),
  MissSpec(String),
  BadContext(String),
  UnknownOpt(String),
  Collision(String)
}

impl<C> StdError for ErrKind<C> { }


impl<C> fmt::Display for ErrKind<C> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match &*self {
      ErrKind::MissArg(s) => {
        f.write_fmt(format_args!("Missing argument ({})", s))
      },
      ErrKind::MissSpec(s) => {
        f.write_fmt(format_args!("Missing argspec; {}", s))
      }
      ErrKind::BadContext(s) => {
        f.write_fmt(format_args!("Bad context; {}", s))
      }
      ErrKind::UnknownOpt(s) => {
        f.write_fmt(format_args!("Unknown option; {}", s))
      }
      ErrKind::Collision(s) => {
        f.write_fmt(format_args!("Colliding options; {}", s))
      }
    }
  }
}

impl<C> fmt::Debug for ErrKind<C> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match &*self {
      ErrKind::MissArg(s) => {
        f.write_fmt(format_args!("Missing argument ({})", s))
      },
      ErrKind::MissSpec(s) => {
        f.write_fmt(format_args!("Missing argspec; {}", s))
      }
      ErrKind::BadContext(s) => {
        f.write_fmt(format_args!("Bad context; {}", s))
      }
      ErrKind::UnknownOpt(s) => {
        f.write_fmt(format_args!("Unknown option; {}", s))
      }
      ErrKind::Collision(s) => {
        f.write_fmt(format_args!("Colliding options; {}", s))
      }
    }
  }
}


// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
