use qargparser as arg;

#[derive(Default, Debug)]
struct MyContext {
  do_help: bool,
  exec: String,
  eargs: Vec<String>
}

fn help_proc(
  _spec: &arg::Spec<MyContext>,
  ctx: &mut MyContext,
  _args: &Vec<String>
) {
  ctx.do_help = true;
}

fn exec_proc(
  _spec: &arg::Spec<MyContext>,
  ctx: &mut MyContext,
  args: &Vec<String>
) {
  ctx.exec = args[0].clone();
}

fn eargs_proc(
  _spec: &arg::Spec<MyContext>,
  ctx: &mut MyContext,
  args: &Vec<String>
) {
  for arg in args {
    ctx.eargs.push(arg.clone());
  }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
  let help_spec = arg::Builder::new()
    .sopt('h')
    .lopt("help")
    .exit(true)
    .help(&["Show this help."])
    .build(help_proc);
  let exec_spec = arg::Builder::new()
    .name("exec")
    .nargs(arg::Nargs::Count(1), &["PRG"])
    .help(&["The executable to run."])
    .build(exec_proc);
  let eargs_spec = arg::Builder::new()
    .name("execargs")
    .nargs(arg::Nargs::Remainder, &["PRGARG"])
    .help(&["arguments to pass to the executable."])
    .build(eargs_proc);

  let ctx = MyContext {
    ..Default::default()
  };
  let mut prsr = arg::Parser::from_env(ctx);

  prsr.add(help_spec)?;
  prsr.add(exec_spec)?;
  prsr.add(eargs_spec)?;

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
