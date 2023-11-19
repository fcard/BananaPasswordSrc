use std::env::args;

pub(self) use super::prelude::*;

mod parse;
mod interactive;

use parse::*;
use interactive::*;


const HELP_MSG: &'static str = "
banana_pass [-i] [-s <world>-<level>] [-t <treasure>] [-w <weapon>] [-p <print_style>]
  -s, --stage:        sets the stage, between 1-1 to 7-3

  -t, --treasure:     sets the amount of treasures, can be a number or name
                        possible values: 0, 1, 2, 3, 4,
                                         none, sword, shield, armor, crown

  -w, --weapon:       sets the current weapon, can be a number or name
                      dashes in names can be substitued by a space or nothing
                        possible values: 0 through 15,
                                         stone-axe, king-axe, star-axe,
                                         magic, capsule, maxim, meteor,
                                         megalo, fold, cross-axe,
                                         grand-axe, curse-axe, mercury,
                                         flash, crusher, kingship

  -p, --print:        sets the print style of the password.
                        possible values:
                          full: default value, prints a verbose password
                          compact: same as full, but with much less spacing
                          numbers: print the password as numbers
                          none: does a dry run, i.e. doesn't print anything

  -i, --interactive:  instead of printing the password once, start an
                      interactive mode. other options will set the
                      initial password.
";

#[derive(Copy, Clone)]
pub(self) enum PrintType {
  None,
  Full,
  Compact,
  Bananas,
  Numbers
}

impl PrintType {
  pub(self) fn print(self, p: &Password, prefix: &str) {
    match self {
      PrintType::None => {},
      PrintType::Full => {
        if prefix.is_empty() {
          println!("{:#?}", p);
        } else {
          for pline in format!("{:#?}", p).lines() {
            println!("{}{}", prefix, pline);
          }
        }
      }


      PrintType::Compact => {
        println!("{}{:?}", prefix, p);
      }

      PrintType::Bananas => {
        println!("{}{:?}", prefix, p.bananas);
      }

      PrintType::Numbers => {
        print!("{}", prefix);

        let mut iter = p.bananas.iter().map(|x| *x as u8);

        if let Some(n0) = iter.next() {
          print!("{:02b}", n0);
          for n in iter {
            print!(" {:02b}", n);
          }
        }
        println!("");
      }
    }
  }
}

#[derive(Clone, Copy)]
enum ArgState {
  Top,
  Stage,
  Treasure,
  Weapon,
  PrintType
}

pub(super) fn cli() {
  let mut p = Password::new();
  let mut state = ArgState::Top;

  let mut stage       = Stage::START;
  let mut treasure    = Treasure::None;
  let mut weapon      = Weapon::StoneAxe;
  let mut interactive = false;
  let mut print_type  = PrintType::Full;

  macro_rules! p {
    ($t: expr, $arg: expr) => {
      panic!("invalid {} = {:?}", $t,
        unsafe { String::from_utf8_unchecked($arg) })
    }
  }

  for a in args().skip(1).map(String::into_bytes)  {
    match state {
      ArgState::Top => {
        if a == b"--interactive" || a == b"-i" {
          interactive = true;

        } else if a == b"--stage" || a == b"-s" {
          state = ArgState::Stage;

        } else if a == b"--treasure" || a == b"-t" {
          state = ArgState::Treasure;

        } else if a == b"--weapon" || a == b"-w" {
          state = ArgState::Weapon;

        } else if a == b"--print" || a == b"-p" {
          state = ArgState::PrintType;

        } else if a == b"--help" || a == b"-h" {
          println!("{}", HELP_MSG);
          return;

        } else if let Some(s) = stage_number(&a) {
          stage = s;

        } else if let Some(t) = treasure_number::<DISALLOW_NUMBER>(&a) {
          treasure = t;

        } else if let Some(w) = weapon_number::<DISALLOW_NUMBER>(&a) {
          weapon = w;

        } else {
          p!("arg", a)
        }
      }

      ArgState::Stage => {
        if let Some(s) = stage_number(&a) {
          stage = s;
          state = ArgState::Top;
        } else {
          p!("stage", a)
        }
      }

      ArgState::Treasure => {
        if let Some(t) = treasure_number::<ALLOW_NUMBER>(&a) {
          treasure = t;
          state = ArgState::Top;
        } else {
          p!("treasure", a)
        }
      }

      ArgState::Weapon => {
        if let Some(w) = weapon_number::<ALLOW_NUMBER>(&a) {
          weapon = w;
          state = ArgState::Top;
        } else {
          p!("weapon", a)
        }
      }

      ArgState::PrintType => {
        if let Some(p) = print_arg(&a) {
          print_type = p;
          state = ArgState::Top;
        } else {
          p!("print type", a)
        }
      }
    }
  }

  p.set_stage(stage);
  p.set_treasure(treasure);
  p.set_weapon(weapon);

  if interactive {
    interactive_mode(&mut p);
  } else {
    print_type.print(&p, "");
  }
}

