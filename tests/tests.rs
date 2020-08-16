use std::cell::RefCell;

use qargparser as arg;

#[cfg(test)]
macro_rules! vec_of_strings {
  ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

#[cfg(test)]
#[derive(Default)]
struct MyContext {
  do_help: bool,
  verbosity: u8,
  fname: String
}

#[cfg(test)]
fn help_proc(
  _spec: &arg::Spec<MyContext>,
  ctx: &mut MyContext,
  _args: &Vec<String>
) {
  ctx.do_help = true;
}

#[cfg(test)]
fn verbose_proc(
  _spec: &arg::Spec<MyContext>,
  ctx: &mut MyContext,
  _args: &Vec<String>
) {
  ctx.verbosity += 1;
}

#[cfg(test)]
fn file_proc(
  _spec: &arg::Spec<MyContext>,
  ctx: &mut MyContext,
  args: &Vec<String>
) {
  ctx.fname = args[0].clone();
}


#[cfg(test)]
fn mkhelp() -> arg::Spec<MyContext> {
  arg::Builder::new()
    .sopt('h')
    .lopt("help")
    .exit(true)
    .build(help_proc)
}

#[cfg(test)]
fn mkverbose() -> arg::Spec<MyContext> {
  arg::Builder::new()
    .sopt('v')
    .lopt("verbose")
    .build(verbose_proc)
}

#[cfg(test)]
fn mkfile() -> arg::Spec<MyContext> {
  arg::Builder::new()
    .sopt('f')
    .lopt("file")
    .nargs(arg::Nargs::Count(1), &["FILE"])
    .build(file_proc)
}

/*
fn mkparam() -> arg::Spec<MyContext> {
  arg::Builder::new().sopt('p').lopt("param")
    .nargs(arg::Nargs::Count(2), &["KEY", "VALUE"]).build(param_proc)
}
*/


#[test]
fn app_test_1() -> Result<(), Box<dyn std::error::Error>> {
  let ctx = MyContext {
    ..Default::default()
  };
  let spec = mkhelp();
  assert_eq!(ctx.do_help, false);

  let argv = vec_of_strings!["--help"];

  let mut prsr = arg::Parser::from_args("cmd", &argv, ctx);

  prsr.add(spec)?;

  prsr.parse()?;

  assert_eq!(prsr.get_ctx().do_help, true);

  Ok(())
}


#[test]
fn app_test_2() -> Result<(), Box<dyn std::error::Error>> {
  let ctx = MyContext {
    ..Default::default()
  };
  let help_spec = mkhelp();
  let verbose_spec = mkverbose();
  let fname_spec = mkfile();

  assert_eq!(ctx.do_help, false);

  let argv = vec_of_strings!["--help"];

  let mut prsr = arg::Parser::from_args("cmd", &argv, ctx);

  prsr.add(help_spec)?;
  prsr.add(verbose_spec)?;
  prsr.add(fname_spec)?;

  prsr.parse()?;

  assert_eq!(prsr.get_ctx().do_help, true);

  Ok(())
}


#[test]
fn app_test_3() -> Result<(), Box<dyn std::error::Error>> {
  let ctx = MyContext {
    ..Default::default()
  };
  let help_spec = mkhelp();
  let verbose_spec = mkverbose();
  let fname_spec = mkfile();

  assert_eq!(ctx.verbosity, 0);
  assert_eq!(ctx.do_help, false);

  let mut prsr = arg::Parser::from_args("cmd", &["--verbose", "-v"], ctx);

  prsr.add(help_spec)?;
  prsr.add(verbose_spec)?;
  prsr.add(fname_spec)?;

  prsr.parse()?;

  assert_eq!(prsr.get_ctx().verbosity, 2);

  Ok(())
}

/* vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 : */
