use qargparser as arg;

#[derive(Default, Debug)]
struct MyContext {
  do_help: bool
}

fn help_proc(
  _spec: &arg::Spec<MyContext>,
  ctx: &mut MyContext,
  _args: &Vec<String>
) {
  ctx.do_help = true;
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
  let help_spec = arg::Builder::new()
    .sopt('h')
    .lopt("help")
    .exit(true)
    .help(&["Show this help."])
    .build(help_proc);
  let coll_spec = arg::Builder::new()
    .sopt('h')
    .lopt("hulp")
    .exit(true)
    .help(&["Show this help."])
    .build(help_proc);


  let ctx = MyContext {
    ..Default::default()
  };
  let mut prsr = arg::Parser::from_env(ctx);

  prsr.add(help_spec)?;
  prsr.add(coll_spec)?;

  prsr.parse()?;

  //let ctx = prsr.into_ctx();
  if prsr.get_ctx().do_help == true {
    prsr.usage(&mut std::io::stdout());
    std::process::exit(0);
  }

  println!("{:?}", prsr.get_ctx());

  Ok(())
}


// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
