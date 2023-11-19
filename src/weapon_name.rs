use attrs::*;
#[web] use with_length::*;
use super::weapon::*;

#[web]
pub(crate) enum WeaponName<const W: Weapon>
where [u8; W.name_len()-1]: {
  SameSize([u8; W.name_len()]),
  Shrunk([u8; W.name_len()-1])
}

#[web]
impl<const W: Weapon> ToString for WeaponName<W>
where [u8; W.name_len()-1]: {
  fn to_string(&self) -> String {
    let v: &[u8] =
      match self {
        WeaponName::SameSize(s) => s as &[u8],
        WeaponName::Shrunk(s)   => s as &[u8]
      };

    unsafe { String::from_utf8_unchecked(Vec::from(v)) }
  }
}

const fn split_point(a: &[u8]) -> Option<(usize, bool)> {
  let mut i = 0;
  while i < a.len() {
    if a[i] == b' ' {
      return Some((i, false));
    } else if a[i] == b'_' {
      return Some((i, true));
    }
    i += 1;
  }
  return None;
}

impl Weapon {
  pub(crate) const NAMES: &'static [&'static str] = &[
    "Stone Axe",
    "King Axe",
    "Star Axe",
    "Magic",
    "Capsule",
    "Maxim",
    "Meteor",
    "Megalo",
    "Fold",
    "Cross_Axe",
    "Grand_Axe",
    "Curse_Axe",
    "Mercury",
    "Flash",
    "Crusher",
    "Kingship"
  ];

  pub(crate) const fn name_len(self) -> usize {
    Weapon::NAMES[self as usize].len()
  }

  #[web]
  pub(crate) fn select_name_rt(self) -> String {
    with_int!(self as usize, W, 0..16, {
      Weapon::select_name::<{Weapon::BY_NUMBER[W]}>().to_string()
    })
  }

  #[web]
  pub(crate) const fn select_name<const W: Weapon>() -> WeaponName<W>
  where [u8; W.name_len()-1]: {
    macro_rules! copy_slice {
      ($a: ident, $b: ident, $i: ident, $j: expr$(, $k: expr)?) => {
        while $i < $j {
          $a[$i] = $b[$i $(+ $k)*].to_ascii_uppercase();
          $i += 1;
        }
      }
    }

    let mut j = 0;
    let name = Weapon::NAMES[W as usize].as_bytes();

    if let Some((i, underscore)) = split_point(name) {
      if underscore {
        let mut result: [u8; W.name_len()-1] = [0; W.name_len()-1];
        copy_slice!(result, name, j, i);
        copy_slice!(result, name, j, W.name_len()-1, 1);
        WeaponName::Shrunk(result)
      } else {
        let mut result: [u8; W.name_len()] = [0; W.name_len()];
        copy_slice!(result, name, j, i);
        result[j] = b' ';
        j += 1;
        copy_slice!(result, name, j, W.name_len());
        WeaponName::SameSize(result)
      }
    } else {
      let mut result: [u8; W.name_len()] = [0; W.name_len()];
      copy_slice!(result, name, j, W.name_len());
      WeaponName::SameSize(result)
    }
  }

  #[cli]
  pub(crate) fn is_command_name<const W: Weapon>(s: &[u8]) -> bool
  where [u8; W.name_len()-1]: {
    macro_rules! cmp {
      ($a: ident, $b: ident, $i: ident, $j: expr$(, $k: expr)?) => {
        if $b.len() < $j {
          return false;
        }

        while $i < $j {
          if !$a[$i $(+ $k)*].eq_ignore_ascii_case(&$b[$i]) {
            return false;
          }
          $i += 1;
        }
      }
    }

    let mut j = 0;
    let name = Weapon::NAMES[W as usize].as_bytes();
    let test = s.trim_ascii();

    if let Some((i, _)) = split_point(name) {
      cmp!(name, test, j, i);
      if j == test.len() {
        return true;
      }

      let k;
      if test[j] == b' ' || test[j] == b'_' {
        k = 0;
        j += 1;
      } else {
        k = 1;
      }

      cmp!(name, test, j, W.name_len()-k, k);
      return j == test.len();
    } else {
      cmp!(name, test, j, W.name_len());
      return j == test.len();
    }
  }
}

