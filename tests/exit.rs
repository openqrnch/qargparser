use std::cell::RefCell;

use qargparser as arg;

#[derive(Default)]
pub struct MyContext {
  do_help: bool,
  fname: String,
  optcount: usize,
  argcount: usize
}

/*
fn opt_proc(_spec: &arg::Spec<MyContext>, ctx: &mut MyContext) {
  ctx.optcount += 1;
}
*/

fn arg_proc(_spec: &arg::Spec<MyContext>, ctx: &mut MyContext,
    _args: &Vec<String>) {
  ctx.argcount += 1;
}

fn help_proc(_spec: &arg::Spec<MyContext>, ctx: &mut MyContext,
    _args: &Vec<String>) {
  ctx.do_help = true;
}

fn file_proc(_spec: &arg::Spec<MyContext>, ctx: &mut MyContext,
    args: &Vec<String>) {
  ctx.fname = args[0].clone();
}



#[cfg(test)]
mod testutil {
  pub fn mkhelp() -> super::arg::Spec<super::MyContext> {
    super::arg::Builder::new()
      .sopt('h')
      .lopt("help")
      .exit(true)
      .build(super::help_proc)
  }
  pub fn mkfile() -> super::arg::Spec<super::MyContext> {
    super::arg::Builder::new()
      .sopt('f')
      .lopt("file")
      .nargs(super::arg::Nargs::Count(1), &["FILE"])
      .build(super::file_proc)
  }
}

/*
macro_rules! mkhelp {
    ($x:expr) => {{xerror($x)}};
    ($x:expr, $($y:expr),+) => {{xerror(&format!($x, $y))}};
}
*/

#[test]
fn exit_advances() -> Result<(), Box<dyn std::error::Error>> {
  let ctx = MyContext{ ..Default::default() };

  let help_spec = testutil::mkhelp();
  let arg1 = arg::Builder::new().name("cmd1")
      .nargs(arg::Nargs::Count(1), &["FIRST"]).required(true)
      .build(arg_proc);
  let arg2 = arg::Builder::new().name("cmd2")
      .nargs(arg::Nargs::Count(1), &["SECOND"])
      .build(arg_proc);
  let arg3 = arg::Builder::new().name("cmd3")
      .nargs(arg::Nargs::Count(1), &["EXIT"])
      .exit(true)
      .build(arg_proc);
  let arg4 = arg::Builder::new().name("cmd4")
      .nargs(arg::Nargs::Count(1), &["FOURTH"])
      .build(arg_proc);

  assert_eq!(ctx.optcount, 0);
  assert_eq!(ctx.argcount, 0);

  let args = ["one", "two", "three", "four"];
  let mut prsr = arg::Parser::from_args("cmd", &args, ctx);

  prsr.add(help_spec)?;
  prsr.add(arg1)?;
  prsr.add(arg2)?;
  prsr.add(arg3)?;
  prsr.add(arg4)?;

  assert_eq!(prsr.num_remaining_args(), 4);
  assert_eq!(prsr.num_remaining_posargspecs(), 4);

  let spec = prsr.parse()?;
  if let Some(ref _spec) = spec {
    assert_eq!(prsr.get_ctx().argcount, 3);
  } else {
    panic!("Unexpected return value from parse()");
  }

  assert_eq!(prsr.num_remaining_args(), 1);
  assert_eq!(prsr.num_remaining_posargspecs(), 1);


  let spec = prsr.parse()?;
  assert_eq!(spec.is_none(), true);
  assert_eq!(prsr.get_ctx().argcount, 4);

  assert_eq!(prsr.num_remaining_args(), 0);
  assert_eq!(prsr.num_remaining_posargspecs(), 0);

  Ok(())
}

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
