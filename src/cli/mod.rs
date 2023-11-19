use std::env::args;

pub(self) use super::prelude::*;

mod parse;
mod interactive;

use parse::*;
use interactive::*;

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

