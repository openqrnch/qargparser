use qargparser as arg;

#[derive(Default, Debug)]
struct MyContext {
  do_help: bool,
  do_version: bool,
  do_secret: bool
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
  let help_spec = arg::Builder::new()
    .sopt('h')
    .lopt("help")
    .exit(true)
    .help(&["Show this help."])
    .build(
      |_spec: &arg::Spec<MyContext>,
       ctx: &mut MyContext,
       _args: &Vec<String>| ctx.do_help = true
    );
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
  let secret_spec = arg::Builder::new()
    .sopt('s')
    .lopt("secret")
    .hidden(true)
    .help(&["A hidden option."])
    .build(
      |_spec: &arg::Spec<MyContext>,
       ctx: &mut MyContext,
       _args: &Vec<String>| ctx.do_secret = true
    );


  let ctx = MyContext {
    ..Default::default()
  };
  let mut prsr = arg::Parser::from_env(ctx);

  prsr.add(help_spec)?;
  prsr.add(version_spec)?;
  prsr.add(secret_spec)?;

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
