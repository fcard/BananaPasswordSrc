use std::iter::Step;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq)]
pub(crate) enum Weapon {
  StoneAxe = 0,
  KingAxe,
  StarAxe,
  Magic,
  Capsule,
  Maxim,
  Meteor,
  Megalo,
  Fold,
  CrossAxe,
  GrandAxe,
  CurseAxe,
  Mercury,
  Flash,
  Crusher,
  KingShip
}
impl std::marker::ConstParamTy for Weapon {}


impl Weapon {
  pub(crate) const BY_NUMBER: &'static [Weapon] = &[
    Weapon::StoneAxe,
    Weapon::KingAxe,
    Weapon::StarAxe,
    Weapon::Magic,
    Weapon::Capsule,
    Weapon::Maxim,
    Weapon::Meteor,
    Weapon::Megalo,
    Weapon::Fold,
    Weapon::CrossAxe,
    Weapon::GrandAxe,
    Weapon::CurseAxe,
    Weapon::Mercury,
    Weapon::Flash,
    Weapon::Crusher,
    Weapon::KingShip
  ];


  pub(crate) fn banana_offsets(self) -> &'static [usize; 8] {
    use Weapon::*;
    match self {
      StoneAxe  => &[0, 0, 0, 0, 0, 0, 0, 0],
      KingAxe   => &[1, 0, 0, 0, 0, 0, 0, 1],
      StarAxe   => &[2, 0, 0, 0, 0, 1, 0, 1],
      Magic     => &[3, 0, 0, 0, 0, 2, 0, 1],
      Capsule   => &[2, 0, 0, 0, 0, 0, 0, 2],
      Maxim     => &[3, 0, 0, 0, 0, 1, 0, 2],
      Meteor    => &[4, 0, 0, 0, 0, 2, 0, 2],
      Megalo    => &[3, 0, 0, 0, 0, 0, 0, 3],
      Fold      => &[4, 0, 0, 0, 0, 1, 0, 3],
      CrossAxe  => &[2, 0, 0, 0, 0, 2, 0, 0],
      GrandAxe  => &[4, 0, 0, 0, 0, 3, 0, 1],
      CurseAxe  => &[1, 0, 0, 0, 0, 1, 0, 0],
      Mercury   => &[5, 0, 0, 0, 0, 2, 0, 3],
      Flash     => &[3, 0, 0, 0, 0, 3, 0, 0],
      Crusher   => &[5, 0, 0, 0, 0, 3, 0, 2],
      KingShip  => &[6, 0, 0, 0, 0, 3, 0, 3],
    }
  }

  pub(crate) fn next(self) -> Self {
    match Self::forward_checked(self, 1) {
      Some(w) => w,
      None    => Weapon::KingShip
    }
  }

  pub(crate) fn prev(self) -> Self {
    match Self::backward_checked(self, 1) {
      Some(w) => w,
      None    => Weapon::StoneAxe
    }
  }
}

impl Step for Weapon {
  fn steps_between(this: &Self, other: &Self) -> Option<usize> {
    let a = *this  as usize;
    let b = *other as usize;
    if b >= a {
      return Some(b - a);
    } else {
      return None;
    }
  }

  fn forward_checked(start: Self, count: usize) -> Option<Self> {
    let i = start as usize;
    return Weapon::BY_NUMBER.get(i + count).copied();
  }

  fn backward_checked(start: Self, count: usize) -> Option<Self> {
    let i = start as usize;
    if i >= count {
      return Some(Weapon::BY_NUMBER[i - count]);
    } else {
      return None;
    }
  }
}

