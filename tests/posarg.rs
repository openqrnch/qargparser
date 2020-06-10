
// Make sure parser croaks if there aren't enough arguments
// Nargs::Count(2)
// Nargs::Count(3)

use std::cell::RefCell;

use qargparser as arg;

#[derive(Default,Debug)]
struct MyContext {
  cmd: String,
  subcmd: String,
}

fn cmd_proc(_spec: &arg::Spec<MyContext>, ctx: &mut MyContext,
    args: &Vec<String>) {
  ctx.cmd = args[0].clone();
  assert_eq!(ctx.cmd, "foo");
}

fn subcmd_proc(_spec: &arg::Spec<MyContext>, ctx: &mut MyContext,
    args: &Vec<String>) {
  ctx.subcmd = args[0].clone();
  assert_eq!(ctx.subcmd, "bar");
}

#[test]
fn posarg() -> Result<(), Box<dyn std::error::Error>> {
  let cmd_spec = arg::Builder::new().name("command").required(true)
      .nargs(arg::Nargs::Count(1), &["COMMAND"])
      .help(&["The command to run."])
      .build(cmd_proc);
  let subcmd_spec = arg::Builder::new().name("subcmd")
      .nargs(arg::Nargs::Count(1), &["SUBCMD"])
      .help(&["Command-specific sub-command."])
      .build(subcmd_proc);

  let ctx = MyContext{..Default::default()};

  let mut prsr = arg::Parser::from_args("posarg", &["foo", "bar"], ctx);

  prsr.add(cmd_spec)?;
  prsr.add(subcmd_spec)?;

  prsr.parse()?;

  Ok(())
}

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
