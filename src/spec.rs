//use std::cmp::Ordering;

#[derive(Copy, Clone)]
pub enum Nargs {
  None,
  Count(usize),
  Remainder /*Optional,
             *ZeroOrMore,
             *OneOrMore */
}

impl Default for Nargs {
  fn default() -> Self {
    Nargs::None
  }
}

type Handler<C> = fn(spec: &Spec<C>, ctx: &mut C, args: &Vec<String>);


/*
type HandlerStr<C> = fn(spec: &Spec<C>, ctx: &mut C, arg: &str);
type HandlerString<C> = fn(spec: &Spec<C>, ctx: &mut C, arg: String);

type HandlerStrVec<C, I, S> = fn(spec: &Spec<C>, ctx: &mut C, args: I)
  where I: IntoIterator<Item=S>,
        S: AsRef<str>;

enum Handler {
  Str(HandlerStr),
  String(HandlerString)
}
*/


/// Parser option/argument specification builder.
//#[derive(Default)]
pub struct Builder {
  sopt: Option<char>,
  lopt: Option<String>,
  name: Option<String>,
  nargs: Nargs,
  exit: bool,
  required: bool,
  metanames: Vec<String>,
  desc: Vec<String>,

  /// Whether to hide this entry from the help text.
  hidden: bool
}

impl Builder {
  pub fn new() -> Self {
    Builder {
      sopt: None,
      lopt: None,
      name: None,
      nargs: Nargs::None,
      exit: false,
      required: false,
      metanames: Vec::new(),
      desc: Vec::new(),
      hidden: false
    }
  }

  /// Make argument specification a single-character option.
  pub fn sopt(&mut self, sopt: char) -> &mut Self {
    self.sopt = Some(sopt);
    self
  }


  /// Make argument specification a long option name.
  pub fn lopt(&mut self, lopt: &str) -> &mut Self {
    self.lopt = Some(String::from(lopt));
    self
  }

  /// Assign argument specification a name.  This is required for positional
  /// arguments.
  pub fn name(&mut self, name: &str) -> &mut Self {
    self.name = Some(String::from(name));
    self
  }

  /// Declare that this argument specification takes arguments.
  pub fn nargs<I, S>(&mut self, nargs: Nargs, metanames: I) -> &mut Self
  where
    I: IntoIterator<Item = S>,
    S: AsRef<str>
  {
    self.nargs = nargs;
    self.metanames(metanames);
    self
  }

  /// Assign meta-names to the arguments.
  pub fn metanames<I, S>(&mut self, metanames: I) -> &mut Self
  where
    I: IntoIterator<Item = S>,
    S: AsRef<str>
  {
    self.metanames.clear();
    let nargs = match self.nargs {
      Nargs::Remainder => 1,
      Nargs::Count(n) => n,
      _ => 0
    };

    let names = metanames
      .into_iter()
      .map(|x| String::from(x.as_ref()))
      .collect::<Vec<_>>();

    if nargs > 0 {
      for i in 0..nargs {
        if i < names.len() {
          self.metanames.push(names[i].clone());
        } else {
          self.metanames.push("ARG".to_string());
        }
      }
    }
    self
  }

  /// Specify if encountering this argument specification should terminate the
  /// parser.
  ///
  /// This is useful for options that should terminate normal program
  /// behavor, such as `--help` or if a positional argumen has been encountered
  /// which should abort the parser because the rest of the arguments should be
  /// parsed by a different parser.
  pub fn exit(&mut self, exit: bool) -> &mut Self {
    self.exit = exit;
    self
  }

  /// Tell the parser that the argument must be processed.  This is only useful
  /// for positional arguments.
  pub fn required(&mut self, req: bool) -> &mut Self {
    self.required = req;
    self
  }

  /// Hidden arguments exist and work as usual, but they are not displayed in
  /// the help screen.
  pub fn hidden(&mut self, hidden: bool) -> &mut Self {
    self.hidden = hidden;
    self
  }

  pub fn help<I, S>(&mut self, text: I) -> &mut Self
  where
    I: IntoIterator<Item = S>,
    S: AsRef<str>
  {
    for x in text.into_iter() {
      //println!("moo: {}", x.as_ref());
      self.desc.push(String::from(x.as_ref()));
      //println!("{:?}", self.desc);
    }

    /*
    self.desc.clear();
    //text.into_iter().map(|x| self.desc.push(String::from(x.as_ref())));
    text.into_iter().map(|x| println!("moo: {:?}", x.as_ref()));
    */

    self
  }

  pub fn build<C>(&self, proc: Handler<C>) -> Spec<C> {
    Spec {
      sopt: self.sopt,
      lopt: self.lopt.clone(),
      name: self.name.clone(),
      nargs: self.nargs.clone(),
      exit: self.exit,
      required: self.required,
      metanames: self.metanames.clone(),
      desc: self.desc.clone(),
      hidden: self.hidden,
      proc
    }
  }
}


/// Option/argument specification.
//#[derive(Default)]
pub struct Spec<C> {
  /// Optional short option.  This must be unique within a `[Parser]` context.
  /// If this is `Some()` value then this is not a positional argument.
  pub(crate) sopt: Option<char>,

  /// Optional long option.  This must be unique within a `[Parser]` context.
  /// If this is `Some()` value then this is not a positional argument.
  pub(crate) lopt: Option<String>,

  /// Optional argument name.  If this is `Some()` value and both `[sopt]` and
  /// `[lopt]` are None then this is a positional argument.
  pub(crate) name: Option<String>,
  nargs: Nargs,
  pub(crate) exit: bool,
  required: bool,
  metanames: Vec<String>,
  desc: Vec<String>,
  hidden: bool,
  pub(crate) proc: Handler<C>
}


impl<C> Spec<C> {
  /// Return a boolean indicating whether this arg spec is configured to
  /// capture all the remaining arguments.
  pub fn is_capture_rest(&self) -> bool {
    match self.nargs {
      Nargs::None => false,
      Nargs::Count(_n) => false,
      Nargs::Remainder => true
    }
  }

  /// Return boolean indicating whether this arg spec will abort the parser.
  pub fn is_exit(&self) -> bool {
    self.exit
  }

  /// Return boolean indicating whether this arg spec represents an "option".
  ///
  /// "Options" come in two forms: short or long
  ///
  /// Short options are in the form "<single dash><single non-dash character>",
  /// like "-h", "-f".
  ///
  /// Long options are in the form "<double dash><word>", like "--help",
  /// --file".
  ///
  /// This function will return true for either form.
  pub fn is_opt(&self) -> bool {
    self.sopt.is_some() || self.lopt.is_some()
  }

  /// Return boolean indicating whether this arg is a positional argument.
  pub fn is_pos(&self) -> bool {
    self.sopt.is_none() && self.lopt.is_none()
  }
  pub fn is_req(&self) -> bool {
    self.required
  }

  pub fn is_hidden(&self) -> bool {
    self.hidden
  }

  // ToDo: Don't panic!(), return Result instead.
  pub fn get_nargs(&self) -> usize {
    match self.nargs {
      Nargs::None => 0,
      Nargs::Count(n) => n,
      Nargs::Remainder => {
        panic!("Can't get number of arguments for a capture-all spec.");
      }
    }
  }
  pub fn req_args(&self) -> bool {
    match self.nargs {
      Nargs::None => false,
      Nargs::Count(n) => {
        if n > 0 {
          return true;
        }
        false
      }
      Nargs::Remainder => false
    }
  }

  /// Generate a string representation of a short option.
  /// Does not include any arguments.
  ///
  /// Examples: "-h", "-f"
  fn get_sopt_str(&self) -> Option<String> {
    if let Some(sopt) = self.sopt {
      let mut ret = '-'.to_string();
      let soptstr = sopt.to_string();
      ret.push_str(&soptstr);
      return Some(ret);
    }
    None
  }

  /// Generate a string representation of a long option.
  /// Does not include any arguments.
  ///
  /// Examples: "--help", "--file"
  fn get_lopt_str(&self) -> Option<String> {
    if let Some(ref lopt) = self.lopt {
      let mut ret = "--".to_string();
      ret.push_str(&lopt);
      return Some(ret);
    }
    None
  }

  fn get_joined_meta_str(&self) -> Option<String> {
    match self.nargs {
      Nargs::None => None,
      Nargs::Count(_n) => Some(self.metanames.join(" ")),
      Nargs::Remainder => {
        // ARG [ARG ...]
        let metaname = if self.metanames.len() > 0 {
          &self.metanames[0]
        } else {
          "ARG"
        };
        Some(metaname.to_string())
      }
    }
  }

  /// Get a short option argument string.
  ///
  /// Example formats:
  /// - Some("-h")
  /// - Some("-f FILE")
  /// - Some("-p XCOORD YCOORD")
  fn get_soptarg_str(&self) -> Option<String> {
    if let Some(optstr) = self.get_sopt_str() {
      let mut ret = optstr.to_owned();
      if let Some(metastr) = self.get_joined_meta_str() {
        ret.push_str(" ");
        ret.push_str(&metastr);
      }
      return Some(ret);
    }
    None
  }
  fn get_loptarg_str(&self) -> Option<String> {
    if let Some(optstr) = self.get_lopt_str() {
      let mut ret = optstr.to_owned();
      if let Some(metastr) = self.get_joined_meta_str() {
        ret.push_str(" ");
        ret.push_str(&metastr);
      }
      return Some(ret);
    }
    None
  }

  #[cfg(test)]
  fn get_sopt_usage_str(&self) -> Option<String> {
    if let Some(optstr) = self.get_soptarg_str() {
      let mut ret: String;
      if self.required {
        ret = String::from("<");
      } else {
        ret = String::from("[");
      }
      ret.push_str(&optstr);
      if self.required {
        ret.push_str(">");
      } else {
        ret.push_str("]");
      }
      return Some(ret);
    }
    None
  }

  #[cfg(test)]
  fn get_lopt_usage_str(&self) -> Option<String> {
    if let Some(optstr) = self.get_loptarg_str() {
      let mut ret: String;
      if self.required {
        ret = String::from("<");
      } else {
        ret = String::from("[");
      }
      ret.push_str(&optstr);
      if self.required {
        ret.push_str(">");
      } else {
        ret.push_str("]");
      }
      return Some(ret);
    }
    None
  }

  // "-f FILE, --file FILE"
  // "-f FILE"
  // "--file FILE"
  pub fn get_opts_usage_str(&self) -> String {
    let mut args: Vec<String> = Vec::new();
    if let Some(lstr) = self.get_soptarg_str() {
      args.push(lstr);
    }
    if let Some(rstr) = self.get_loptarg_str() {
      args.push(rstr);
    }
    if args.len() == 0 {
      if let Some(posarg) = self.get_joined_meta_str() {
        args.push(posarg);
      }
    }
    return args.join(", ");
  }


  /// Generate a string that shows a representation of this argument
  /// specification suitable for use in a "Usage: " string.
  ///
  /// Required parameters are enclosed by '<' and '>' charcters.
  /// Optional parameters are enclossed by '[' and ']' characters.
  pub fn get_usage_str(&self) -> String {
    let mut ret: String;
    if self.required {
      ret = '<'.to_string();
    } else {
      ret = '['.to_string();
    }

    if let Some(optstr) = self.get_loptarg_str() {
      ret.push_str(&optstr);
    } else if let Some(optstr) = self.get_soptarg_str() {
      ret.push_str(&optstr);
    } else if let Some(metastr) = self.get_joined_meta_str() {
      let s = match self.nargs {
        Nargs::Count(_) => metastr.clone(),
        Nargs::Remainder => format!("{0} [{0} ...]", metastr),
        _ => panic!(
          "Attempted to print positional argument spec with no arguments"
        )
      };
      ret.push_str(&s);
    } else {
      panic!("Unexpected state");
    }

    if self.required {
      ret.push_str(">");
    } else {
      ret.push_str("]");
    }

    return ret;
  }

  pub fn get_help_title_str(&self) -> String {
    let mut args: Vec<String> = Vec::new();
    if let Some(lstr) = self.get_soptarg_str() {
      args.push(lstr);
    }
    if let Some(rstr) = self.get_loptarg_str() {
      args.push(rstr);
    }
    if args.len() == 0 {
      if let Some(posarg) = self.get_joined_meta_str() {
        let s = match self.nargs {
          Nargs::Count(_) => posarg.clone(),
          Nargs::Remainder => posarg.clone(),
          _ => panic!(
            "Attempted to print positional argument spec with no arguments"
          )
        };
        args.push(s);
      }
    }
    return args.join(", ");
  }


  pub fn get_help_text(&self) -> &Vec<String> {
    &self.desc
  }
}


/*
 * Sort order
 * - Compare sopt to sopt
 * - Compare lopt to lopt
 * - Some(sopt) < lopt:None
 * - posarg > optarg
 * - posargs compare their index
 */
/*
impl<C> Default for Spec<C> {
  fn default() -> Self {
    let metanames = Vec::new();
    let desc = Vec::new();
    Spec { sopt: None, lopt: None, name: None, nargs: Nargs::None,
        exit: false, required: false, metanames, desc, proc: Callback::None }
  }
}
*/


/*
impl<C> PartialEq for Spec<C> {
  fn eq(&self, other: &Spec<C>) -> bool {
    true
  }
}

impl<C> PartialOrd for Spec<C> {
  fn partial_cmp(&self, other: &Spec<C>) -> Option<Ordering> {
    if self.is_pos() && !other.is_pos() {
      return Ordering::Less;
    } else if !self.is_pos() && other.is_pos() {
      return Ordering::Greater;
    }
    Ordering::Equal
  }
}
*/


#[cfg(test)]
mod tests {
  use std::collections::HashMap;

  #[derive(Default)]
  pub(super) struct TestCtx {
    pub(super) do_help: bool,
    pub(super) verbosity: u8,
    pub(super) fname: String,
    pub(super) params: HashMap<String, String>
  }
  pub(super) fn help_proc(
    _spec: &super::Spec<TestCtx>,
    ctx: &mut TestCtx,
    _args: &Vec<String>
  ) {
    ctx.do_help = true;
  }
  pub(super) fn verbose_proc(
    _spec: &super::Spec<TestCtx>,
    ctx: &mut TestCtx,
    _args: &Vec<String>
  ) {
    ctx.verbosity += 1;
  }
  pub(super) fn file_proc(
    _spec: &super::Spec<TestCtx>,
    ctx: &mut TestCtx,
    args: &Vec<String>
  ) {
    ctx.fname = args[0].clone();
  }
  pub(super) fn param_proc(
    _spec: &super::Spec<TestCtx>,
    ctx: &mut TestCtx,
    args: &Vec<String>
  ) {
    ctx.params.insert(args[0].clone(), args[1].clone());
  }
  pub(super) fn args_proc(
    _spec: &super::Spec<TestCtx>,
    _ctx: &mut TestCtx,
    _args: &Vec<String>
  ) {
  }


  pub(super) fn mkhelp(sopt: bool, lopt: bool) -> super::Spec<TestCtx> {
    let mut bldr = super::Builder::new();
    if sopt {
      bldr.sopt('h');
    }
    if lopt {
      bldr.lopt("help");
    }
    bldr.build(help_proc)
  }
  pub(super) fn mkfile(
    sopt: bool,
    lopt: bool,
    name: bool,
    argname: bool
  ) -> super::Spec<TestCtx> {
    let mut bldr = super::Builder::new();
    if sopt {
      bldr.sopt('f');
    }
    if lopt {
      bldr.lopt("file");
    }
    if name {
      bldr.name("file");
    }
    if argname {
      bldr.nargs(super::Nargs::Count(1), &["FILE"]);
    } else {
      let nm: Vec<String> = Vec::new();
      bldr.nargs(super::Nargs::Count(1), &nm);
    }
    bldr.build(file_proc)
  }

  pub(super) fn mkparam(
    sopt: bool,
    lopt: bool,
    name: bool,
    defargs: usize
  ) -> super::Spec<TestCtx> {
    let mut bldr = super::Builder::new();
    if sopt {
      bldr.sopt('p');
    }
    if lopt {
      bldr.lopt("param");
    }
    if name {
      bldr.name("param");
    }
    if defargs > 1 {
      bldr.nargs(super::Nargs::Count(2), &["KEY", "VALUE"]);
    } else if defargs == 1 {
      bldr.nargs(super::Nargs::Count(1), &["KEY"]);
    } else {
      let nm: Vec<String> = Vec::new();
      bldr.nargs(super::Nargs::Count(0), &nm);
    }
    bldr.build(param_proc)
  }
}


// Make this a macro?
#[cfg(test)]
fn expect_opt_str(optstr: &Option<String>, expect: &str) {
  if let Some(optstr) = optstr {
    assert_eq!(optstr, expect);
  } else {
    panic!("This shouldn't be possbile.");
  }
}


#[test]
fn test_metanames() {
  let spec = Builder::new()
    .sopt('h')
    .lopt("help")
    .exit(true)
    .build(tests::help_proc);

  // No meta names for plain switches
  assert_eq!(spec.get_joined_meta_str(), None);

  let spec = Builder::new()
    .sopt('f')
    .lopt("file")
    .name("file")
    .nargs(Nargs::Count(1), &["FILE"])
    .build(tests::file_proc);

  expect_opt_str(&spec.get_joined_meta_str(), "FILE");
  expect_opt_str(&spec.get_joined_meta_str(), "FILE");
  expect_opt_str(&spec.get_soptarg_str(), "-f FILE");
  expect_opt_str(&spec.get_loptarg_str(), "--file FILE");


  let nm: Vec<String> = Vec::new();
  let spec = Builder::new()
    .sopt('f')
    .lopt("file")
    .name("file")
    .nargs(Nargs::Count(1), &nm)
    .build(tests::file_proc);

  expect_opt_str(&spec.get_joined_meta_str(), "ARG");
  expect_opt_str(&spec.get_joined_meta_str(), "ARG");
  expect_opt_str(&spec.get_soptarg_str(), "-f ARG");
  expect_opt_str(&spec.get_loptarg_str(), "--file ARG");


  let spec = super::Builder::new()
    .sopt('p')
    .lopt("param")
    .name("param")
    .nargs(super::Nargs::Count(2), &["KEY", "VALUE"])
    .build(tests::param_proc);

  expect_opt_str(&spec.get_joined_meta_str(), "KEY VALUE");


  let spec = super::Builder::new()
    .sopt('p')
    .lopt("param")
    .name("param")
    .nargs(super::Nargs::Count(2), &["KEY"])
    .build(tests::param_proc);

  expect_opt_str(&spec.get_joined_meta_str(), "KEY ARG");


  let nm: Vec<String> = Vec::new();
  let spec = super::Builder::new()
    .sopt('p')
    .lopt("param")
    .name("param")
    .nargs(super::Nargs::Count(2), &nm)
    .build(tests::param_proc);

  expect_opt_str(&spec.get_joined_meta_str(), "ARG ARG");
}


#[test]
fn test_help_switch() {
  let spec = Builder::new()
    .sopt('h')
    .lopt("help")
    .exit(true)
    .build(tests::help_proc);

  assert_eq!(spec.is_exit(), true);

  expect_opt_str(&spec.get_sopt_str(), "-h");
  expect_opt_str(&spec.get_lopt_str(), "--help");

  expect_opt_str(&spec.get_soptarg_str(), "-h");
  expect_opt_str(&spec.get_loptarg_str(), "--help");

  expect_opt_str(&spec.get_sopt_usage_str(), "[-h]");
  expect_opt_str(&spec.get_lopt_usage_str(), "[--help]");

  // No meta names for plain switches
  assert_eq!(spec.get_joined_meta_str(), None);
}

#[test]
fn test_switch_noshort() {
  let spec = Builder::new()
    .lopt("help")
    .exit(true)
    .build(tests::help_proc);

  assert_eq!(spec.is_exit(), true);

  let soptstr = spec.get_sopt_str();
  assert_eq!(soptstr.is_none(), true);
}


#[test]
fn test_switch_nolong() {
  let spec = Builder::new().sopt('h').exit(true).build(tests::help_proc);

  assert_eq!(spec.is_exit(), true);

  let loptstr = spec.get_lopt_str();
  assert_eq!(loptstr.is_none(), true);
}


#[test]
fn test_file_optarg() {
  let spec = Builder::new()
    .sopt('f')
    .lopt("file")
    .name("file")
    .nargs(Nargs::Count(1), &["FILE"])
    .build(tests::file_proc);


  expect_opt_str(&spec.get_joined_meta_str(), "FILE");
  expect_opt_str(&spec.get_soptarg_str(), "-f FILE");
  expect_opt_str(&spec.get_loptarg_str(), "--file FILE");


  let nm: Vec<String> = Vec::new();
  let spec = Builder::new()
    .sopt('f')
    .lopt("file")
    .name("file")
    .nargs(Nargs::Count(1), &nm)
    .build(tests::file_proc);

  expect_opt_str(&spec.get_joined_meta_str(), "ARG");
  expect_opt_str(&spec.get_soptarg_str(), "-f ARG");
  expect_opt_str(&spec.get_loptarg_str(), "--file ARG");
}

#[test]
fn test_param_optarg() {
  let spec = super::Builder::new()
    .sopt('p')
    .lopt("param")
    .name("param")
    .nargs(super::Nargs::Count(2), &["KEY", "VALUE"])
    .build(tests::param_proc);

  expect_opt_str(&spec.get_joined_meta_str(), "KEY VALUE");

  let spec = super::Builder::new()
    .sopt('p')
    .lopt("param")
    .name("param")
    .nargs(super::Nargs::Count(2), &["KEY"])
    .build(tests::param_proc);

  expect_opt_str(&spec.get_joined_meta_str(), "KEY ARG");


  let nm: Vec<String> = Vec::new();
  let spec = super::Builder::new()
    .sopt('p')
    .lopt("param")
    .name("param")
    .nargs(super::Nargs::Count(2), &nm)
    .build(tests::param_proc);

  expect_opt_str(&spec.get_joined_meta_str(), "ARG ARG");
}


/*

#[test]
fn test_optarg_usage_str()
{
  let spec = tests::mkfile(true, true, false, true);
  expect_opt_str(&spec.get_sopt_usage_str(), "[-f FILE]");
  expect_opt_str(&spec.get_lopt_usage_str(), "[--file FILE]");

}

#[test]
fn test_opts_usage_str()
{
  let spec = tests::mkhelp(true, true);
  assert_eq!(spec.get_opts_usage_str(), "-h, --help");

  let spec = tests::mkhelp(false, true);
  assert_eq!(spec.get_opts_usage_str(), "--help");

  let spec = tests::mkhelp(true, false);
  assert_eq!(spec.get_opts_usage_str(), "-h");

  let spec = tests::mkfile(true, true, false, true);
  assert_eq!(&spec.get_opts_usage_str(), "-f FILE, --file FILE");

  let spec = tests::mkfile(false, true, false, true);
  assert_eq!(&spec.get_opts_usage_str(), "--file FILE");

  let spec = tests::mkfile(true, false, false, true);
  assert_eq!(&spec.get_opts_usage_str(), "-f FILE");
}

#[test]
fn test_usage_str()
{
  let spec = tests::mkhelp(true, true);
  assert_eq!(spec.get_usage_str(), "[--help]");

  let spec = tests::mkhelp(false, true);
  assert_eq!(spec.get_usage_str(), "[--help]");

  let spec = tests::mkhelp(true, false);
  assert_eq!(spec.get_usage_str(), "[-h]");
}

#[test]
fn test_usage_req_str()
{
  let mut spec = Spec::new_opt(Some('h'), Some("help"), tests::help_proc);
  spec.set_required(true);
  assert_eq!(spec.get_usage_str(), "<--help>");

  let mut spec = Spec::new_opt(None, Some("help"), tests::help_proc);
  spec.set_required(true);
  assert_eq!(spec.get_usage_str(), "<--help>");

  let mut spec = Spec::new_opt(Some('h'), None::<String>, tests::help_proc);
  spec.set_required(true);
  assert_eq!(spec.get_usage_str(), "<-h>");
}


#[test]
fn test_usage_arg_str()
{
  let spec = Spec::new_argopt(Some('f'), Some("file"), Nargs::Count(1),
      &["FILE"], tests::file_proc);
  assert_eq!(spec.get_usage_str(), "[--file FILE]");

  let spec = Spec::new_argopt(None, Some("file"), Nargs::Count(1),
      &["FILE"], tests::file_proc);
  assert_eq!(spec.get_usage_str(), "[--file FILE]");

  let spec = Spec::new_argopt(Some('f'), None::<String>, Nargs::Count(1),
      &["FILE"], tests::file_proc);
  assert_eq!(spec.get_usage_str(), "[-f FILE]");
}

#[test]
fn test_usage_arg_req_str()
{
  let mut spec = Spec::new_argopt(Some('f'), Some("file"), Nargs::Count(1),
      &["FILE"], tests::file_proc);
  spec.set_required(true);
  assert_eq!(spec.get_usage_str(), "<--file FILE>");

  let mut spec = Spec::new_argopt(None, Some("file"), Nargs::Count(1),
      &["FILE"], tests::file_proc);
  spec.set_required(true);
  assert_eq!(spec.get_usage_str(), "<--file FILE>");

  let mut spec = Spec::new_argopt(Some('f'), None::<String>, Nargs::Count(1),
      &["FILE"], tests::file_proc);
  spec.set_required(true);
  assert_eq!(spec.get_usage_str(), "<-f FILE>");
}


#[test]
fn test_usage_posarg_str()
{
  let spec = Spec::new_posarg("cmd", Nargs::Count(1), &["COMMAND"],
      tests::args_proc);
  assert_eq!(spec.get_usage_str(), "[COMMAND]");

  let spec = Spec::new_posarg("cmd", Nargs::Count(2), &["COMMAND"],
      tests::args_proc);
  assert_eq!(spec.get_usage_str(), "[COMMAND ARG]");
}

#[test]
fn test_usage_posarg_req_str()
{
  let mut spec = Spec::new_posarg("cmd", Nargs::Count(1), &["COMMAND"],
      tests::args_proc);
  spec.set_required(true);
  assert_eq!(spec.get_usage_str(), "<COMMAND>");

  let mut spec = Spec::new_posarg("cmd", Nargs::Count(2), &["COMMAND"],
      tests::args_proc);
  spec.set_required(true);
  assert_eq!(spec.get_usage_str(), "<COMMAND ARG>");
}

*/


// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
