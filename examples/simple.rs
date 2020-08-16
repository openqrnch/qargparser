use std::collections::HashMap;

use qargparser as arg;

#[derive(Default, Debug)]
struct MyContext {
  do_help: bool,
  do_version: bool,
  verbosity: u8,
  fname: String,
  params: HashMap<String, String>,
  cmd: String,
  subcmd: String
}

fn help_proc(
  _spec: &arg::Spec<MyContext>,
  ctx: &mut MyContext,
  _args: &Vec<String>
) {
  ctx.do_help = true;
}

fn verbose_proc(
  _spec: &arg::Spec<MyContext>,
  ctx: &mut MyContext,
  _args: &Vec<String>
) {
  ctx.verbosity += 1;
}

/*
fn version_proc(_spec: &arg::Spec<MyContext>, ctx: &mut MyContext,
    _args: &Vec<String>) {
  ctx.do_version = true;
}
*/

fn file_proc(
  _spec: &arg::Spec<MyContext>,
  ctx: &mut MyContext,
  args: &Vec<String>
) {
  ctx.fname = args[0].clone();
}

fn param_proc(
  _spec: &arg::Spec<MyContext>,
  ctx: &mut MyContext,
  args: &Vec<String>
) {
  ctx.params.insert(args[0].clone(), args[1].clone());
}

fn cmd_proc(
  _spec: &arg::Spec<MyContext>,
  ctx: &mut MyContext,
  args: &Vec<String>
) {
  ctx.cmd = args[0].clone();
}

fn subcmd_proc(
  _spec: &arg::Spec<MyContext>,
  ctx: &mut MyContext,
  args: &Vec<String>
) {
  ctx.subcmd = args[0].clone();
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
  let help_spec = arg::Builder::new()
    .sopt('h')
    .lopt("help")
    .exit(true)
    .help(&["Show this help."])
    .build(help_proc);
  let verbose_spec = arg::Builder::new()
    .sopt('v')
    .lopt("verbose")
    .help(&["Increase level of verbosity.", "Be quiet by default."])
    .build(verbose_proc);
  let version_spec = arg::Builder::new()
    .sopt('V')
    .lopt("version")
    .exit(true)
    .help(&["Output tool version and exit."])
    .build(
      |_spec: &arg::Spec<MyContext>,
       ctx: &mut MyContext,
       _args: &Vec<String>| ctx.do_version = true
    );
  let file_spec = arg::Builder::new()
    .sopt('f')
    .lopt("file")
    .help(&["Use data in FILE."])
    .nargs(arg::Nargs::Count(1), &["FILE"])
    .build(file_proc);
  let param_spec = arg::Builder::new()
    .sopt('p')
    .lopt("param")
    .help(&["Add a key/value parameter field. The key must be unique."])
    .nargs(arg::Nargs::Count(2), &["KEY", "VALUE"])
    .build(param_proc);
  let cmd_spec = arg::Builder::new()
    .name("command")
    .required(true)
    .nargs(arg::Nargs::Count(1), &["COMMAND"])
    .help(&["The command to run."])
    .build(cmd_proc);
  let subcmd_spec = arg::Builder::new()
    .name("subcmd")
    .nargs(arg::Nargs::Count(1), &["SUBCMD"])
    .help(&["Command-specific sub-command."])
    .build(subcmd_proc);


  let ctx = MyContext {
    ..Default::default()
  };
  let mut prsr = arg::Parser::from_env(ctx);

  prsr.add(help_spec)?;
  prsr.add(verbose_spec)?;
  prsr.add(version_spec)?;
  prsr.add(file_spec)?;
  prsr.add(param_spec)?;
  prsr.add(cmd_spec)?;
  prsr.add(subcmd_spec)?;

  prsr.parse()?;

  if prsr.get_ctx().do_help == true {
    prsr.usage(&mut std::io::stdout());
    std::process::exit(0);
  }

  let ctx = prsr.into_ctx();

  println!("{:?}", &ctx);

  Ok(())
}

/* vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 : */
