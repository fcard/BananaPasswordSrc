use super::*;
use crate::weapon::Weapon;

fn stage_from_number<T, S>(world: T, level: S) -> Stage
where T: Into<usize>,
      S: Into<usize>
{
  Stage {
    level: level.into(),
    world: world.into()
  }
}

pub(crate) fn stage_number(s: &[u8]) -> Option<Stage> {
  if s.len() == 3 &&
     s[0] >= b'1' &&
     s[0] <= b'7' &&
     s[1] == b'-' &&
     s[2] >= b'1' &&
     s[2] <= b'3' {
    return Some(stage_from_number(s[0] - b'0', s[2] - b'0'));
  } else {
    return None;
  }
}

pub(crate) type AllowNumber = bool;
pub(crate) const ALLOW_NUMBER: AllowNumber = true;
pub(crate) const DISALLOW_NUMBER: AllowNumber = false;

fn treasure_from_number<T: Into<usize>>(n: T) -> Treasure {
  Treasure::BY_NUMBER[n.into()]
}

pub(crate) fn treasure_number<const ALLOW_NUM: AllowNumber>
(s: &[u8]) -> Option<Treasure> {
  if ALLOW_NUM && s.len() == 1 &&
     s[0] >= b'0' &&
     s[0] <= b'4' {
    return Some(treasure_from_number(s[0] - b'0'));

  } else if s == b"none" {
    return Some(Treasure::None);

  } else if s == b"sword" {
    return Some(Treasure::Sword);

  } else if s == b"shield" {
    return Some(Treasure::Shield);

  } else if s == b"armor" {
    return Some(Treasure::Armor);

  } else if s == b"crown" || s == b"all" {
    return Some(Treasure::Crown);

  } else {
    return None;
  }
}

fn weapon_from_number<T: Into<usize>>(n: T) -> Option<Weapon> {
  Weapon::BY_NUMBER.get(n.into()).copied()
}

pub(crate) fn weapon_number<const ALLOW_NUM: AllowNumber>
(s: &[u8]) -> Option<Weapon> {
  if ALLOW_NUM && s.len() == 1 && s[0].is_ascii_digit() {
    return weapon_from_number(s[0] - b'0');

  } else if ALLOW_NUM && s.len() == 2 &&
    s[0].is_ascii_digit() &&
    s[1].is_ascii_digit() {
    return weapon_from_number((10*(s[0] - b'0') + (s[1] - b'0')) as usize);

  } else {
    weapon_number_named(s)
  }
}

fn weapon_number_named(s: &[u8]) -> Option<Weapon> {
  use Weapon::*;

  macro_rules! cmp {
    ($w: ident) => {
      Weapon::is_command_name::<{Weapon::$w}>(s)
    }
  }

  if s == b"none"            ||
     s == b"start"           ||
     s == b"starting"        ||
     s == b"weakest"         ||
     s == b"min"             ||
     s == b"minimum"         ||
     s == b"starting weapon" ||
     cmp!(StoneAxe) {
    return Some(StoneAxe);

  } else if cmp!(StarAxe) {
    return Some(StarAxe);

  } else if cmp!(Magic) {
    return Some(Magic);

  } else if cmp!(Capsule) {
    return Some(Capsule);

  } else if cmp!(Maxim) {
    return Some(Maxim);

  } else if cmp!(Meteor) {
    return Some(Meteor);

  } else if cmp!(Megalo) {
    return Some(Megalo);

  } else if cmp!(Fold) {
    return Some(Fold);

  } else if cmp!(GrandAxe) {
    return Some(GrandAxe);

  } else if cmp!(CurseAxe) {
    return Some(CurseAxe);

  } else if cmp!(Mercury) {
    return Some(Mercury);

  } else if cmp!(Flash) {
    return Some(Flash);

  } else if cmp!(Crusher) {
    return Some(Crusher);

  } else if s == b"max"        ||
            s == b"maximum"    ||
            s == b"strongest"  ||
            s == b"final"      ||
            cmp!(KingShip) {
    return Some(KingShip)

  } else {
    return None;
  }
}


pub(super) fn print_arg(s: &[u8]) -> Option<PrintType> {
  match s {
    b"full" | b"default" | b"f" => { Some(PrintType::Full) }
    b"comp" | b"compact" | b"c" => { Some(PrintType::Compact) }
    b"none" | b"0"    => { Some(PrintType::None) }
    b"bananas" | b"b" => { Some(PrintType::Bananas) }
    b"numbers" | b"n" => { Some(PrintType::Numbers) }
    _ => {
      return None;
    }
  }
}
