use std::cell::RefCell;

use qargparser as arg;

#[derive(Default)]
struct MyContext {
  fname: String,
  pidfile: String
}

fn file_proc(
  _spec: &arg::Spec<MyContext>,
  ctx: &mut MyContext,
  args: &Vec<String>
) {
  ctx.fname = args[0].clone();
  assert_eq!(ctx.fname, "test.txt");
}

fn pidfile_proc(
  _spec: &arg::Spec<MyContext>,
  ctx: &mut MyContext,
  args: &Vec<String>
) {
  ctx.pidfile = args[0].clone();
  assert_eq!(ctx.pidfile, "foobar.pid");
}

#[test]
fn optarg() -> Result<(), Box<dyn std::error::Error>> {
  let file_spec = arg::Builder::new()
    .sopt('f')
    .lopt("file")
    .help(&["Use file FILE."])
    .nargs(arg::Nargs::Count(1), &["FILE"])
    .build(file_proc);
  let pidfile_spec = arg::Builder::new()
    .sopt('p')
    .lopt("pidfile")
    .help(&["Store process pid in FILE."])
    .nargs(arg::Nargs::Count(1), &["FILE"])
    .build(pidfile_proc);

  let ctx = MyContext {
    ..Default::default()
  };

  let mut prsr = arg::Parser::from_args(
    "optarg",
    &["--pidfile", "foobar.pid", "--file", "test.txt"],
    ctx
  );

  prsr.add(file_spec)?;
  prsr.add(pidfile_spec)?;

  prsr.parse()?;

  Ok(())
}

/* vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 : */
