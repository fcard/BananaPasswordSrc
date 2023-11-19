#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Treasure {
  None,
  Sword,
  Shield,
  Armor,
  Crown
}

impl Treasure {
  pub(crate) const BY_NUMBER: &'static [Treasure] = &[
    Treasure::None,
    Treasure::Sword,
    Treasure::Shield,
    Treasure::Armor,
    Treasure::Crown
  ];

  pub(crate) fn get_next(&mut self) {
    use Treasure::*;
    match *self {
      None   => { *self = Sword; },
      Sword  => { *self = Shield; },
      Shield => { *self = Armor; },
      Armor  => { *self = Crown; },
      Crown  => {}
    }
  }

  pub(crate) fn lose(&mut self) {
    use Treasure::*;
    match *self {
      None   => {},
      Sword  => { *self = None; },
      Shield => { *self = Sword; },
      Armor  => { *self = Shield; },
      Crown  => { *self = Armor; }
    }
  }
}

