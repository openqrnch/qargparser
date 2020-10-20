use qargparser as arg;

#[derive(Default)]
pub struct MyContext {
  pub exec: String,
  pub eargs: Vec<String>,
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

#[test]
fn postest() -> Result<(), Box<dyn std::error::Error>> {
  let ctx = MyContext {
    ..Default::default()
  };

  let exec_spec = arg::Builder::new()
  .nargs(arg::Nargs::Count(1), &["PRG"])
  .build(exec_proc);

let eargs_spec = arg::Builder::new()
  .nargs(arg::Nargs::Remainder, &["PRGARG"])
  .build(eargs_proc);

  let args = ["Powershell.exe", "--", "-ExecutionPolicy"];
  let mut prsr = arg::Parser::from_args("cmd", &args, ctx);
  //let mut prsr = arg::Parser::from_env(ctx);

  prsr.add(exec_spec)?;
  prsr.add(eargs_spec)?;

  prsr.parse()?;

  let ctx = prsr.into_ctx();

  assert_eq!(ctx.exec, "Powershell.exe");
  assert_eq!(ctx.eargs, vec!["-ExecutionPolicy"]);

  Ok(())
}