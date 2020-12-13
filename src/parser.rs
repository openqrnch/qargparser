use std::collections::HashMap;
use std::rc::Rc;
//use std::cell::{RefCell, Ref, RefMut};
use std::cell::RefCell;
use std::env;

use qpprint as pprint;

use crate::prsrutil;
use crate::spec::Spec;

use crate::err::{ErrKind, SpecErr};


/// The core parser.
pub struct Parser<C> {
  ctx: C,
  specs: Vec<Rc<RefCell<Spec<C>>>>,
  sopts: HashMap<char, Rc<RefCell<Spec<C>>>>,
  lopts: HashMap<String, Rc<RefCell<Spec<C>>>>,
  named: HashMap<String, Rc<RefCell<Spec<C>>>>,
  posargs: Vec<Rc<RefCell<Spec<C>>>>,
  argv0: String,
  args: Vec<String>,
  curarg: usize,
  posplit: bool,
  posarg: usize,
  err: Option<ErrKind<C>>,
  tophelp: Vec<String>,
  bottomhelp: Vec<String>
}

impl<C> Parser<C> {
  /// Create a parser for parsing the process' command line arguments.
  pub fn from_env(ctx: C) -> Self {
    let args: Vec<String> = env::args().collect();
    let args2 = &args[1..];

    Parser::from_args(&args[0], &args2.to_vec(), ctx)
  }

  pub fn from_args<I, S>(argv0: &str, args: I, ctx: C) -> Self
  where
    I: IntoIterator<Item = S>,
    S: AsRef<str>
  {
    let specs = Vec::new();
    let sopts = HashMap::new();
    let lopts = HashMap::new();
    let posargs = Vec::new();
    let named = HashMap::new();
    let new_args = args
      .into_iter()
      .map(|x| String::from(x.as_ref()))
      .collect::<Vec<_>>();

    Parser {
      specs,
      sopts,
      lopts,
      posargs,
      named,
      argv0: String::from(argv0),
      args: new_args,
      ctx,
      curarg: 0,
      posplit: false,
      posarg: 0,
      err: None,
      tophelp: Vec::new(),
      bottomhelp: Vec::new()
    }
  }

  pub fn add(&mut self, spec: Spec<C>) -> Result<(), ErrKind<C>> {
    //let aspec_rc: Rc<RefCell<ArgSpec>> = Rc::new(RefCell::new(argspec));
    let aspec_rc = Rc::new(RefCell::new(spec));
    let asp = aspec_rc.borrow();

    //
    // Make sure this spec is unique
    //
    if let Some(ref c) = asp.sopt {
      if self.sopts.contains_key(c) {
        let errstr = format!("The short option '{}' already in use.", c);
        return Err(ErrKind::Collision(errstr));
      }
    }
    if let Some(ref o) = asp.lopt {
      if self.lopts.contains_key(o) {
        let errstr = format!("The long option '{}' already in use.", o);
        return Err(ErrKind::Collision(errstr));
      }
    }
    if let Some(ref n) = asp.name {
      if self.named.contains_key(n) {
        let errstr =
          format!("The positional argument '{}' already in use.", n);
        return Err(ErrKind::Collision(errstr));
      }
    }

    //
    // Add spec to parser context
    //

    // Add to list of all specs
    self.specs.push(Rc::clone(&aspec_rc));

    if let Some(ref c) = asp.sopt {
      self.sopts.insert(*c, Rc::clone(&aspec_rc));
    }

    if let Some(ref o) = asp.lopt {
      self.lopts.insert(o.clone(), Rc::clone(&aspec_rc));
    }

    if let Some(ref n) = asp.name {
      self.named.insert(n.clone(), Rc::clone(&aspec_rc));
    }

    // If it's neither a long or short option then it's a positional argument.
    if asp.is_pos() == true {
      // Make sure the last positional argument spec doesn't capture "the
      // rest".
      if self.have_capture_rest() {
        return Err(ErrKind::BadContext(
          "Can't add positional argument after
existing 'capture all' argument"
            .to_string()
        ));
      }
      self.posargs.push(Rc::clone(&aspec_rc));
    }
    Ok(())
  }

  pub fn get_arg0(&self) -> &str {
    &self.argv0
  }

  pub fn get_remaining_args(&mut self) -> Vec<String> {
    let mut remain = Vec::new();

    for i in self.curarg..self.args.len() {
      remain.push(self.args[i].to_string());
    }

    // ToDo
    // let remain: Vec<String> = self.args.iter().skip(self.curarg).collect();

    remain
  }

  pub fn have_capture_rest(&self) -> bool {
    if let Some(ref spec) = &self.posargs.last() {
      let spec = spec.borrow();
      if spec.is_capture_rest() {
        return true;
      }
    }
    false
  }

  pub fn set_tophelp<I, S>(&mut self, p: I)
  where
    I: IntoIterator<Item = S>,
    S: ToString
  {
    self.tophelp = p.into_iter().map(|x| x.to_string()).collect::<Vec<_>>();
  }


  pub fn append_tophelp<I, S>(&mut self, p: I)
  where
    I: IntoIterator<Item = S>,
    S: ToString
  {
    let mut ps = p.into_iter().map(|x| x.to_string()).collect::<Vec<_>>();
    self.tophelp.append(&mut ps);
  }


  pub fn set_bottomhelp<I, S>(&mut self, p: I)
  where
    I: IntoIterator<Item = S>,
    S: ToString
  {
    self.bottomhelp = p.into_iter().map(|x| x.to_string()).collect::<Vec<_>>();
  }


  pub fn append_bottomhelp<I, S>(&mut self, p: I)
  where
    I: IntoIterator<Item = S>,
    S: ToString
  {
    let mut ps = p.into_iter().map(|x| x.to_string()).collect::<Vec<_>>();
    self.bottomhelp.append(&mut ps);
  }


  pub fn parse(&mut self) -> Result<Option<Rc<RefCell<Spec<C>>>>, ErrKind<C>> {
    /*
        for n in self {
          let spec = n.borrow();
          if spec.exit == true {
            return Ok(Some(Rc::clone(&n)));
          }
        }
    */
    while let Ok(Some(n)) = self.next() {
      let spec = n.borrow();
      if spec.exit == true {
        return Ok(Some(Rc::clone(&n)));
      }
    }

    self.validate()?;

    Ok(None)
  }


  pub fn next(&mut self) -> Result<Option<Rc<RefCell<Spec<C>>>>, ErrKind<C>> {
    if self.curarg == self.args.len() {
      return Ok(None);
    }

    if self.args[self.curarg] == "--" {
      self.posplit = true;
      self.curarg = self.curarg + 1;
      if self.curarg == self.args.len() {
        return Ok(None);
      }
    }

    let ret: Option<Rc<RefCell<Spec<C>>>>;
    let mut args: Vec<String> = Vec::new();

    if !self.posplit && prsrutil::maybe_lopt(&self.args[self.curarg]) {
      match self.proc_lopt(&mut args) {
        Ok(spec) => ret = Some(spec),
        Err(err) => return Err(err)
      }
    } else if !self.posplit && prsrutil::maybe_sopt(&self.args[self.curarg]) {
      match self.proc_sopt(&mut args) {
        Ok(spec) => ret = Some(spec),
        Err(err) => return Err(err)
      }
    } else {
      match self.proc_posarg(&mut args) {
        Ok(spec) => ret = Some(spec),
        Err(err) => return Err(err)
      }
    }

    if let Some(ref spec) = ret {
      let spec = spec.borrow();

      // Call the argspec's callback function
      (spec.proc)(&*spec, &mut self.ctx, &args);
    }

    self.curarg += 1;

    return Ok(ret);
  }


  fn proc_sopt(
    &mut self,
    args: &mut Vec<String>
  ) -> Result<Rc<RefCell<Spec<C>>>, ErrKind<C>> {
    let spec_ref: Rc<RefCell<Spec<C>>>;

    // ["-vfbar"] -> ["-v", "-f", "bar"]
    prsrutil::split_sopts_arg(&mut self.args, self.curarg, &self.sopts);

    // This is excessive -- should probably be getting the nth() character
    // instead, but this leads to having to deal with Option<> instead.
    let sopt: Vec<char> = self.args[self.curarg].chars().collect();
    let sopt = &sopt[1];
    let spec = self.sopts.get(sopt);
    if let Some(spec) = spec {
      spec_ref = Rc::clone(spec);

      //self.curarg += 1;
      match self.copyout_args(&spec_ref, args) {
        Ok(_) => {}
        Err(err) => {
          return Err(err);
        }
      }
    } else {
      let errstr = format!("Unknown short option '{}'", sopt);
      return Err(ErrKind::UnknownOpt(errstr));
    }

    Ok(spec_ref)
  }


  fn proc_lopt(
    &mut self,
    args: &mut Vec<String>
  ) -> Result<Rc<RefCell<Spec<C>>>, ErrKind<C>> {
    let spec_ref: Rc<RefCell<Spec<C>>>;

    // ["--foo=bar"] -> ["--foo", "bar"]
    prsrutil::split_lopt(&mut self.args, self.curarg);

    let lopt = &self.args[self.curarg][2..];
    let spec = self.lopts.get(lopt);
    if let Some(spec) = spec {
      spec_ref = Rc::clone(spec);

      //self.curarg += 1;
      match self.copyout_args(&spec_ref, args) {
        Ok(_) => {}
        Err(err) => {
          return Err(err);
        }
      }
    } else {
      let errstr = format!("Unknown long option '{}'", lopt);
      return Err(ErrKind::UnknownOpt(errstr));
    }

    Ok(spec_ref)
  }


  fn proc_posarg(
    &mut self,
    args: &mut Vec<String>
  ) -> Result<Rc<RefCell<Spec<C>>>, ErrKind<C>> {
    // Make sure there's an argspecs to handle this argument
    if self.posarg == self.posargs.len() {
      return Err(ErrKind::MissSpec(
        "Out of positional argument specs argument".to_string()
      ));
    }

    let spec_ref = Rc::clone(&self.posargs[self.posarg]);

    match self.copyout_args(&spec_ref, args) {
      Ok(_) => {}
      Err(_err) => {}
    }

    self.posarg += 1;

    Ok(spec_ref)
  }


  // If this argspec has arguments, then copy arguments to an argument vector.
  fn copyout_args(
    &mut self,
    spec_rc: &Rc<RefCell<Spec<C>>>,
    args: &mut Vec<String>
  ) -> Result<(), ErrKind<C>> {
    let spec = spec_rc.borrow();

    // If this is is a "capture the rest of the arguments" spec, then captire
    // the rest of the arguments.
    if spec.is_capture_rest() {
      while self.curarg < self.args.len() {
        args.push(self.args[self.curarg].clone());
        self.curarg += 1;
      }

      // Avoid indexing past the end of the vector later on.
      self.curarg -= 1;

      return Ok(());
    }

    // If this argspec requires arguments then make sure there are enough
    // arguments remaining.
    if !prsrutil::check_req_arg_count(
      &self.args,
      self.curarg,
      &spec,
      spec.is_opt()
    ) {
      let err = SpecErr {
        spec: Rc::clone(spec_rc),
        msg: "Missing expected argument.".to_string()
      };
      return Err(ErrKind::MissArg(err));
    }

    let mut nargs = spec.get_nargs();
    while nargs != 0 {
      if spec.is_pos() == false {
        self.curarg += 1;
      }
      args.push(self.args[self.curarg].clone());
      if spec.is_pos() == true {
        self.curarg += 1;
      }
      nargs -= 1;
    }

    // Special case: Positional arguments don't have an option argument
    if spec.is_pos() {
      self.curarg -= 1;
    }

    Ok(())
  }


  //#[cfg(test)]
  pub fn num_remaining_args(&self) -> usize {
    self.args.len() - self.curarg
  }


  //#[cfg(test)]
  pub fn num_remaining_posargspecs(&self) -> usize {
    self.posargs.len() - self.posarg
  }


  fn get_opts(&self) -> Vec<Rc<RefCell<Spec<C>>>> {
    self
      .specs
      .iter()
      .filter(|x| {
        let x = x.borrow();
        x.is_opt()
      })
      .map(|n| Rc::clone(n))
      .collect()
  }


  fn get_posargs(&self) -> Vec<Rc<RefCell<Spec<C>>>> {
    self
      .specs
      .iter()
      .filter(|x| {
        let x = x.borrow();
        x.is_pos()
      })
      .map(|n| Rc::clone(n))
      .collect()
  }


  pub fn validate(&self) -> Result<(), ErrKind<C>> {
    for i in self.posarg..self.posargs.len() {
      let spec = self.posargs[i].borrow();
      if spec.is_req() {
        let err = SpecErr {
          spec: Rc::clone(&self.posargs[i]),
          msg: "Missing required positional argument.".to_string()
        };
        return Err(ErrKind::MissArg(err));
      }
    }
    Ok(())
  }


  /// Print out help text for parser options.
  ///
  /// Hidden options will not be displayed.
  ///
  /// The overall format for the help text is:
  /// ```plain
  /// Usage: <cmd> [options] [positional arguments]
  ///
  /// [top help]
  ///
  /// [options]
  ///
  /// [positional arguments]
  ///
  /// [bottom help]
  /// ```
  pub fn usage(&self, out: &mut dyn std::io::Write) {
    self.print_usage(out);

    if !self.tophelp.is_empty() {
      println!("");
    }
    self.print_tophelp(out);

    self.print_opts(out);
    self.print_posargs(out);

    if !self.bottomhelp.is_empty() {
      println!("");
    }

    self.print_bottomhelp(out);
  }


  /// Print the "Usage" part of the help.
  ///
  /// The output format is:
  ///
  /// ```plain
  /// Usage: <command> [arguments] [positional arguments]
  /// ```
  pub fn print_usage(&self, out: &mut dyn std::io::Write) {
    let mut sv = Vec::new();
    let mut pp = pprint::PPrint::new();

    sv.push(String::from("Usage:"));
    sv.push(self.argv0.clone());

    /*
    self.specs.iter().map(|n| {
      let n = n.borrow();
      sv.push(n.get_usage_str());
    });
    */
    for n in &self.specs {
      let n = n.borrow();
      if n.is_hidden() {
        continue;
      }
      sv.push(n.get_usage_str());
    }

    /*
    let sv: Vec<String> = self.get_opts().iter().map(|n| {
      let n = n.borrow();
      //sv.push(n.get_usage_str());
      return n.get_usage_str();
    }).collect();
    */

    pp.set_indent(7).set_hang(-7);
    pp.print_words(out, &sv);
  }


  pub fn print_tophelp(&self, out: &mut dyn std::io::Write) {
    if self.tophelp.is_empty() {
      return;
    }

    let pp = pprint::PPrint::new();
    for p in &self.tophelp {
      pp.print_p(out, p);
    }
  }

  pub fn print_bottomhelp(&self, out: &mut dyn std::io::Write) {
    if self.bottomhelp.is_empty() {
      return;
    }

    let pp = pprint::PPrint::new();
    for p in &self.bottomhelp {
      pp.print_p(out, p);
    }
  }


  /// Print the help section for "options".  Options are arguments that
  /// have a short and/or long option name.
  pub fn print_opts(&self, out: &mut dyn std::io::Write) {
    let mut pp = pprint::PPrint::new();

    let opts = self.get_opts();
    if opts.len() == 0 {
      return;
    }

    out.write(b"\noptions:\n").expect("Unable to write output.");

    for spec in &opts {
      let spec = spec.borrow();
      if spec.is_hidden() {
        continue;
      }
      pp.set_indent(2);
      pp.print_p(out, &spec.get_opts_usage_str());
      pp.set_indent(4);
      pp.print_plist(out, spec.get_help_text());
    }
  }

  /// Print the help section for positional arguments.
  pub fn print_posargs(&self, out: &mut dyn std::io::Write) {
    let mut pp = pprint::PPrint::new();

    let specs = self.get_posargs();
    if specs.len() == 0 {
      return;
    }

    out
      .write(b"\narguments:\n")
      .expect("Unable to write output.");

    for spec in &specs {
      let spec = spec.borrow();
      pp.set_indent(2);
      pp.print_p(out, &spec.get_help_title_str());
      pp.set_indent(4);
      pp.print_plist(out, spec.get_help_text());
    }
  }


  pub fn did_fail(&self) -> bool {
    self.err.is_some()
  }

  pub fn get_ctx(&self) -> &C {
    &self.ctx
  }

  pub fn into_ctx(self) -> C {
    self.ctx
  }
}

impl<C> Iterator for Parser<C> {
  type Item = Rc<RefCell<Spec<C>>>;

  fn next(&mut self) -> Option<Self::Item> {
    match self.next() {
      Ok(res) => {
        return res;
      }
      Err(err) => {
        self.err = Some(err);
        return None;
      }
    }
  }
}


/* vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 : */
