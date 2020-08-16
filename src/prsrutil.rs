use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
//use std::env;

use crate::spec::Spec;

//use crate::err::{ErrKind};

#[cfg(test)]
use crate::spec::{Builder, Nargs};

#[cfg(test)]
macro_rules! vec_of_strings {
  ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

/// Determine whether an arguments vector element looks like it could be a
/// long option.
pub(crate) fn maybe_lopt(arg: &str) -> bool {
  if arg.len() > 2 && arg.starts_with("--") == true {
    return true;
  }
  false
}

#[test]
fn test_maybe_lopt() {
  assert_eq!(maybe_lopt("foo"), false);
  assert_eq!(maybe_lopt("-f"), false);
  assert_eq!(maybe_lopt("--"), false);
  assert_eq!(maybe_lopt("--foo"), true);
  assert_eq!(maybe_lopt("--f"), true);
  assert_eq!(maybe_lopt(" --foo"), false);
}


/// Determine whether an arguments vector element looks like it could be a
/// short option.
pub(crate) fn maybe_sopt(arg: &str) -> bool {
  if arg.len() > 1 && arg.starts_with("-") == true {
    return true;
  }
  false
}

#[test]
fn test_maybe_sopt() {
  assert_eq!(maybe_sopt("foo"), false);
  assert_eq!(maybe_sopt("-"), false);
  assert_eq!(maybe_sopt("-a"), true);
}


/*
/// Determine if "--" has been encountered.
fn is_end_of_opts(arg: &str) -> bool {
  return arg == "--";
}
*/


pub(crate) fn split_sopts_arg<C>(
  args: &mut Vec<String>,
  argidx: usize,
  sopts: &HashMap<char, Rc<RefCell<Spec<C>>>>
) {
  let curarg = &args[argidx][1..].to_string().clone();
  let chars: Vec<char> = curarg.chars().collect();
  let mut optarg: Option<String> = None;

  let mut idx: usize = 0;
  while idx < chars.len() {
    let opt_spec = sopts.get(&chars[idx]);
    if let Some(ref spec_rc) = opt_spec {
      let spec = spec_rc.borrow();
      if spec.req_args() {
        // spec has arguments -- break out of split loop
        idx += 1;
        break;
      }
    } else {
      panic!("Unknown option");
    }
    idx += 1;
  }

  // If the loop broke out before reaching the end then interpret that as there
  // being an trailing argument.
  if idx != curarg.len() {
    optarg = Some(curarg[idx..].to_string());
  }

  if idx > 1 || optarg.is_some() {
    // Remove the transformed argument
    args.remove(argidx);

    let mut i: usize = 0;
    while i < idx {
      let opt = "-".to_string() + &chars[i].to_string();
      args.insert(argidx + i, opt);
      i += 1;
    }

    if let Some(optarg) = optarg {
      args.insert(argidx + i, optarg);
    }
  }
}


#[cfg(test)]
mod tests {
  #[derive(Default)]
  pub(super) struct TestCtx {
    pub(super) do_help: bool,
    pub(super) verbosity: u8,
    pub(super) fname: String
  }
  pub(super) fn help_proc(
    _spec: &super::Spec<TestCtx>,
    ctx: &mut TestCtx,
    _args: &Vec<String>
  ) {
    ctx.do_help = true;
  }
  pub(super) fn verbose_proc(
    _spec: &super::Spec<TestCtx>,
    ctx: &mut TestCtx,
    _args: &Vec<String>
  ) {
    ctx.verbosity += 1;
  }
  pub(super) fn file_proc(
    _spec: &super::Spec<TestCtx>,
    ctx: &mut TestCtx,
    args: &Vec<String>
  ) {
    ctx.fname = args[0].clone();
  }
}


#[test]
fn test_split_sopt1() {
  //let mut ctx = tests::TestCtx{ ..Default::default() };
  let mut sopts: HashMap<char, Rc<RefCell<Spec<tests::TestCtx>>>> =
    HashMap::new();
  let spec = Builder::new()
    .sopt('h')
    .lopt("help")
    .build(tests::help_proc);

  sopts.insert('h', Rc::new(RefCell::new(spec)));

  let mut args = vec_of_strings!["-h"];
  split_sopts_arg(&mut args, 0, &sopts);
  assert_eq!(args.len(), 1);
  assert_eq!(args[0], "-h");
}

#[test]
fn test_split_sopt2() {
  //let mut _ctx = tests::TestCtx{ ..Default::default() };
  let mut sopts: HashMap<char, Rc<RefCell<Spec<tests::TestCtx>>>> =
    HashMap::new();

  let spec_f = Builder::new()
    .sopt('f')
    .lopt("file")
    .nargs(Nargs::Count(1), &["FILE"])
    .build(tests::file_proc);
  let spec_v = Builder::new()
    .sopt('v')
    .lopt("verbose")
    .build(tests::verbose_proc);

  sopts.insert('f', Rc::new(RefCell::new(spec_f)));
  sopts.insert('v', Rc::new(RefCell::new(spec_v)));

  let mut args = vec_of_strings!["-v", "-f", "bar"];
  split_sopts_arg(&mut args, 0, &sopts);
  assert_eq!(args.len(), 3);
  assert_eq!(args[0], "-v");
  assert_eq!(args[1], "-f");
  assert_eq!(args[2], "bar");

  split_sopts_arg(&mut args, 1, &sopts);
  assert_eq!(args.len(), 3);
  assert_eq!(args[0], "-v");
  assert_eq!(args[1], "-f");
  assert_eq!(args[2], "bar");
}


#[test]
fn test_split_sopt3() {
  //let mut _ctx = tests::TestCtx{ ..Default::default() };
  let mut sopts: HashMap<char, Rc<RefCell<Spec<tests::TestCtx>>>> =
    HashMap::new();
  let spec_f = Builder::new()
    .sopt('f')
    .lopt("file")
    .nargs(Nargs::Count(1), &["ARG"])
    .build(tests::file_proc);

  sopts.insert('f', Rc::new(RefCell::new(spec_f)));

  let mut args = vec_of_strings!["-fbar"];
  //println!("{:?}", args);

  split_sopts_arg(&mut args, 0, &sopts);
  assert_eq!(args.len(), 2);
  assert_eq!(args[0], "-f");
  assert_eq!(args[1], "bar");
}


#[test]
fn test_split_sopt4() {
  //let mut _ctx = tests::TestCtx{ ..Default::default() };
  let mut sopts: HashMap<char, Rc<RefCell<Spec<tests::TestCtx>>>> =
    HashMap::new();

  let spec_f = Builder::new()
    .sopt('f')
    .lopt("file")
    .nargs(Nargs::Count(1), &["ARG"])
    .build(tests::file_proc);
  let spec_v = Builder::new()
    .sopt('v')
    .lopt("verbose")
    .build(tests::verbose_proc);

  sopts.insert('f', Rc::new(RefCell::new(spec_f)));
  sopts.insert('v', Rc::new(RefCell::new(spec_v)));

  let mut args = vec_of_strings!["-vfbar"];
  split_sopts_arg(&mut args, 0, &sopts);
  assert_eq!(args.len(), 3);
  assert_eq!(args[0], "-v");
  assert_eq!(args[1], "-f");
  assert_eq!(args[2], "bar");
}


/// Ensure that there are sufficient arguments remaining for argspec to
/// process.
pub(crate) fn check_req_arg_count<C>(
  args: &Vec<String>,
  idx: usize,
  spec: &Spec<C>,
  offset: bool
) -> bool {
  if spec.get_nargs() != 0 {
    let nremain = if offset == true {
      args.len() - idx - 1
    } else {
      args.len() - idx
    };
    if nremain < spec.get_nargs() {
      return false;
    }
  }
  true
}


// --file=foo  -->  --file foo
pub(crate) fn split_lopt(argv: &mut Vec<String>, i: usize) {
  match argv[i].find('=') {
    None => (),
    Some(idx) => {
      if idx == 0 || idx == argv[i].len() - 1 {
        ()
      }

      let l = argv[i][..idx].to_string();
      let r = argv[i][idx + 1..].to_string();

      argv[i] = l;
      argv.insert(i + 1, r);
    }
  }
}

#[test]
fn test_split_lopt_eq() {
  let mut argv = vec_of_strings!["--foo=bar"];

  split_lopt(&mut argv, 0);
  assert_eq!(argv.len(), 2);
  assert_eq!(argv[0], "--foo");
  assert_eq!(argv[1], "bar");
}

#[test]
fn test_split_lopt_neq() {
  let mut argv = vec_of_strings!["--foo", "bar"];

  split_lopt(&mut argv, 0);
  assert_eq!(argv.len(), 2);
  assert_eq!(argv[0], "--foo");
  assert_eq!(argv[1], "bar");
}

#[test]
fn test_split_lopt_eq_left() {
  let mut argv = vec_of_strings!["--foo=bar", "--baz"];

  split_lopt(&mut argv, 0);
  assert_eq!(argv.len(), 3);
  assert_eq!(argv[0], "--foo");
  assert_eq!(argv[1], "bar");
  assert_eq!(argv[2], "--baz");
}

#[test]
fn test_split_lopt_eq_middle() {
  let mut argv = vec_of_strings!["--moo", "--foo=bar", "--baz"];

  split_lopt(&mut argv, 1);
  assert_eq!(argv.len(), 4);
  assert_eq!(argv[0], "--moo");
  assert_eq!(argv[1], "--foo");
  assert_eq!(argv[2], "bar");
  assert_eq!(argv[3], "--baz");
}

#[test]
fn test_split_lopt_eq_right() {
  let mut argv = vec_of_strings!["--moo", "--foo=bar"];

  split_lopt(&mut argv, 1);
  assert_eq!(argv.len(), 3);
  assert_eq!(argv[0], "--moo");
  assert_eq!(argv[1], "--foo");
  assert_eq!(argv[2], "bar");
}

/* vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 : */
