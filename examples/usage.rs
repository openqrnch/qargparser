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
  subcmd: String,
  tophelp: bool,
  bottomhelp: bool
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
  let help_spec = arg::Builder::new()
    .sopt('h')
    .lopt("help")
    .exit(true)
    .help(&["Show this help."])
    .build(|_spec, ctx: &mut MyContext, _args| {
      ctx.do_help = true;
    });
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
  let tophelp_spec = arg::Builder::new()
    .sopt('t')
    .lopt("tophelp")
    .help(&["Show top help."])
    .build(|_spec, ctx: &mut MyContext, _args| {
      ctx.tophelp = true;
    });
  let bottomhelp_spec = arg::Builder::new()
    .sopt('b')
    .lopt("bottomhelp")
    .help(&["Show bottom help."])
    .build(|_spec, ctx: &mut MyContext, _args| {
      ctx.bottomhelp = true;
    });

  let ctx = MyContext {
    ..Default::default()
  };
  let mut prsr = arg::Parser::from_env(ctx);

  prsr.add(help_spec)?;
  prsr.add(version_spec)?;
  prsr.add(tophelp_spec)?;
  prsr.add(bottomhelp_spec)?;

  prsr.parse()?;

  if prsr.get_ctx().do_help == true {
    prsr.usage(&mut std::io::stdout());
    std::process::exit(0);
  }

  if prsr.get_ctx().do_version == true {
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    println!("usage {}", VERSION);
    std::process::exit(0);
  }

  let ctx = prsr.into_ctx();

  println!("{:?}", &ctx);

  usage_test(ctx)?;

  Ok(())
}


fn usage_test(ctx: MyContext) -> Result<(), Box<dyn std::error::Error>> {
  let help_spec = arg::Builder::new()
    .sopt('h')
    .lopt("help")
    .exit(true)
    .help(&["Show this help."])
    .build(|_spec, ctx: &mut MyContext, _args| {
      ctx.do_help = true;
    });
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
    .build(|_spec, ctx: &mut MyContext, args| {
      ctx.fname = args[0].clone();
    });
  let param_spec = arg::Builder::new()
    .sopt('p')
    .lopt("param")
    .help(&["Add a key/value parameter field. The key must be unique."])
    .nargs(arg::Nargs::Count(2), &["KEY", "VALUE"])
    .build(|_spec, ctx: &mut MyContext, args| {
      ctx.params.insert(args[0].clone(), args[1].clone());
    });
  let cmd_spec = arg::Builder::new()
    .name("command")
    .required(true)
    .nargs(arg::Nargs::Count(1), &["COMMAND"])
    .help(&["The command to run."])
    .build(|_spec, ctx: &mut MyContext, args| {
      ctx.cmd = args[0].clone();
    });
  let subcmd_spec = arg::Builder::new()
    .name("subcmd")
    .nargs(arg::Nargs::Count(1), &["SUBCMD"])
    .help(&["Command-specific sub-command."])
    .build(|_spec, ctx: &mut MyContext, args| {
      ctx.subcmd = args[0].clone();
    });

  let ctx2 = MyContext {
    ..Default::default()
  };
  let mut prsr = arg::Parser::from_args("hello", &["--help"], ctx2);

  if ctx.tophelp {
    prsr.set_tophelp(&[
      "\"You know,\" said Arthur, \"it's at times like this, when I'm \
       trapped in a Vogon airlock with a man from Betelgeuse, and about to \
       die of asphyxiation in deep space that I really wish I'd listened to \
       what my mother told me when I was young.\"",
      "\"Why, what did she tell you?\"",
      "\"I don't know, I didn't listen.\""
    ]);
  }
  if ctx.bottomhelp {
    prsr.set_bottomhelp(&[
      "\"So this is it,\" said Arthur, \"We are going to die.\"",
      "\"Yes,\" said Ford, \"except... no! Wait a minute!\" He suddenly \
       lunged across the chamber at something behind Arthur's line of \
       vision. \"What's this switch?\" he cried.",
      "\"What? Where?\" cried Arthur, twisting round.",
      "\"No, I was only fooling,\" said Ford, \"we are going to die after \
       all.\""
    ]);
  }

  prsr.add(help_spec)?;
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

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
