use super::*;
use super::parse::*;
use std::io;
use std::io::Write;

macro_rules! err {
  ($fmt: expr $(,$s: expr)*) => {
    format!($fmt $(,
      {
        let s0: &[u8] = $s;
        let mut v = Vec::with_capacity(s0.len());
        v.extend_from_slice(s0);
        unsafe { String::from_utf8_unchecked(v) }
      }
    )*)
  }
}

pub(crate) fn interactive_mode(p: &mut Password) {
  let stdin = io::stdin();
  let mut stdout = io::stdout();
  let mut line  = String::new();
  let mut ptype = PrintType::Full;
  let mut quit = false;

  loop {
    print!("> ");
    if let Err(e) = stdout.flush() {
      eprintln!("stdout error: {:?}", e);
    }

    if let Err(e) = stdin.read_line(&mut line) {
      eprintln!("stdin error: {:?}", e);
    } else {
      let bytes = line.as_bytes().trim_ascii();

      if let Err(err) = {|| {
        let _ =
        bytes.len() == 0 ||

        cmd(p, bytes, &[b"stage", b"s"],
            Password::next_stage, Password::prev_stage,
            Password::set_stage, stage_number)? ||

        cmd(p, bytes, &[b"treasure", b"t"],
            Password::get_treasure, Password::lose_treasure,
            Password::set_treasure, treasure_number::<ALLOW_NUMBER>)? ||

        cmd(p, bytes, &[b"weapon", b"w"],
            Password::next_weapon, Password::prev_weapon,
            Password::set_weapon, weapon_number::<ALLOW_NUMBER>)? || {

          if bytes == b"exit" || bytes == b"quit" {
            quit = true;

          } else if let Some(subcmd) = get_subcmd(bytes, &[b"print", b"pt"]) {
            if subcmd == b"reset" || subcmd == b"default" {
              ptype = PrintType::Full;
            } else if let Some(arg) = get_subcmd(subcmd, &[b"set"])  {
              if let Some(p) = print_arg(arg) {
                ptype = p;
              } else {
                return Err(err!("| invalid print type: {}", arg));
              }
            } else {
              return Err(err!("| invalid print command: {}", subcmd));
            }

          } else if let Some(subcmd) =
              get_subcmd(bytes, &[b"password", b"p"]) && subcmd.len() == 9 {

            if subcmd[0] == b'#' {
              for i in 0..=7 {
                p.bananas[i] = match subcmd[i+1] {
                  b'0' => Banana::Unpeeled,
                  b'1' => Banana::HalfPeeled,
                  b'2' => Banana::Peeled,
                  b'3' => Banana::Eaten,
                  _ => {
                    return Err(err!("| invalid password numbers: {}", subcmd));
                  }
                }
              }
            } else {
              for i in 0..=7 {
                p.bananas[i] = match subcmd[i] {
                  b'u' => Banana::Unpeeled,
                  b'h' => Banana::HalfPeeled,
                  b'p' => Banana::Peeled,
                  b'e' => Banana::Eaten,
                  _ => {
                    return Err(err!("| invalid password: {}", subcmd));
                  }
                }
              }
            }

            p.stage = Stage::START;
            p.treasure = Treasure::None;
            p.weapon = Weapon::StoneAxe;
            let mut p0 = p.clone();

            while p0.bananas[3] != Banana::Unpeeled ||
                  p0.bananas[2] != Banana::Unpeeled {
              p.stage.next();
              p0.banana_sub(0);
              p0.banana_sub(3);
            }

            while p0.bananas[1] != Banana::Unpeeled ||
                  p0.bananas[6] != Banana::Unpeeled {
              p.treasure.get_next();
              p0.banana_sub(0);
              p0.banana_sub(6);
            }

            let mut b: [usize; 8] = p0.bananas.map(|x| x as usize);
            b[0] += b[5] * 3;
            b[5] = 0;

            for w in Weapon::StoneAxe..=Weapon::KingShip {
              if w.banana_offsets() == &b {
                p.weapon = w;
                break;
              }
            }

          } else {
            return Err(err!("| invalid command: {}", bytes));
          }
          true
        };
        return Ok(());
      }}() {
        println!("{}", err);
      } else {
        ptype.print(&p, "| ");
      }
    }
    line.clear();
    if quit {
      return;
    }
  }
}

fn get_subcmd<'a>
(line: &'a [u8], cmds: &[&[u8]]) -> Option<&'a [u8]> {
  for cmd in cmds {
    let cmd_len = cmd.len();
    if line.len() >= cmd_len+1 &&
       line.starts_with(cmd)   &&
       line[cmd_len].is_ascii_whitespace() {
      for i in cmd_len+1..line.len() {
        if !line[i].is_ascii_whitespace() {
          return Some(&line[i..]);
        }
      }
      return None;
    }
  }
  return None;
}


fn cmd<T, FNext, FPrev, FSet, FGet>
  (p: &mut Password, bytes: &[u8], c: &[&[u8]],
   next: FNext, prev: FPrev, set: FSet, get: FGet) -> Result<bool,String>
  where FNext: Fn(&mut Password),
        FPrev: Fn(&mut Password),
        FSet:  Fn(&mut Password, T),
        FGet:  Fn(&[u8]) -> Option<T> {
  if let Some(subcmd) = get_subcmd(bytes, c) {
    if subcmd == b"next" {
      next(p);

    } else if subcmd.iter().all(|c| *c == b'>') {
      for _ in 0..subcmd.len() {
        next(p);
      }

    } else if subcmd == b"prev" || subcmd == b"previous" {
      prev(p);

    } else if subcmd.iter().all(|c| *c == b'<') {
      for _ in 0..subcmd.len() {
        prev(p);
      }

    } else if let Some(arg) = get_subcmd(subcmd, &[b"set", b"="]) {
      if let Some(s) = get(arg) {
        set(p, s);
      } else {
        return Err(err!("| invalid {}: {}", c[0], arg));
      }

    } else {
      return Err(err!("| invalid {} command: {}", c[0], subcmd));
    }
    return Ok(true);
  } else {
    return Ok(false);
  }
}

