use super::banana::*;
use super::stage::*;
use super::treasure::*;
use super::weapon::*;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Password {
  pub(crate) bananas: [Banana; 8],
  pub(crate) stage: Stage,
  pub(crate) treasure: Treasure,
  pub(crate) weapon: Weapon
}

impl Default for Password {
  fn default() -> Self {
    Password::new()
  }
}


impl Password {
  pub(crate) fn new() -> Password {
    Password {
      bananas: [Banana::Unpeeled; 8],
      stage: Stage::START,
      treasure: Treasure::None,
      weapon: Weapon::StoneAxe
    }
  }

  pub(crate) fn set_stage(&mut self, s: Stage) {
    let move_stage =
      if self.stage < s { Self::next_stage }
         else           { Self::prev_stage };

    while self.stage != s {
      move_stage(self);
    }
  }

  pub(crate) fn next_stage(&mut self) {
    if self.stage != Stage::END {
      self.stage.next();
      self.banana_add(0);
      self.banana_add(3);
    }
  }

  pub(crate) fn prev_stage(&mut self) {
    if self.stage != Stage::START {
      self.stage.prev();
      self.banana_sub(0);
      self.banana_sub(3);
    }
  }

  pub(crate) fn set_treasure(&mut self, t: Treasure) {
    let move_treasure =
      if self.treasure < t { Self::get_treasure  }
         else              { Self::lose_treasure };

    while self.treasure != t {
      move_treasure(self);
    }
  }

  pub(crate) fn get_treasure(&mut self) {
    if self.treasure != Treasure::Crown {
      self.treasure.get_next();
      self.banana_add(0);
      self.banana_add(6);
    }
  }

  pub(crate) fn lose_treasure(&mut self) {
    if self.treasure != Treasure::None {
      self.treasure.lose();
      self.banana_sub(0);
      self.banana_sub(6);
    }
  }

  pub(crate) fn set_weapon(&mut self, w: Weapon) {
    if self.weapon != Weapon::StoneAxe &&
       self.weapon != w {
      let offsets = self.weapon.banana_offsets();
      for i in 0..=7 {
        for _ in 0..offsets[i] {
          self.banana_sub(i);
        }
      }
    }

    self.weapon = w;
    let offsets = w.banana_offsets();
    for i in 0..=7 {
      for _ in 0..offsets[i] {
        self.banana_add(i);
      }
    }
  }

  #[cfg_attr(target_family = "wasm", allow(unused))]
  pub(crate) fn next_weapon(&mut self) {
    self.weapon = self.weapon.next();
    self.set_weapon(self.weapon);
  }

  #[cfg_attr(target_family = "wasm", allow(unused))]
  pub(crate) fn prev_weapon(&mut self) {
    self.weapon = self.weapon.prev();
    self.set_weapon(self.weapon);
  }

  pub(crate) fn banana_add(&mut self, index: usize) {
    if self.bananas[index].add() == BananaOverflow::Yes {
      match index {
        0 => {
          self.banana_add(4);
        }

        2 => {
          self.banana_add(0);
          self.banana_add(1);
          self.banana_sub(4);
        }

        3 => {
          self.banana_add(0);
          self.banana_add(2);
          self.banana_sub(4);
        }

        6 => {
          self.banana_sub(0);
          self.banana_sub(0);
          self.banana_add(1);
          self.banana_add(1);
        }

        _ => {
          unreachable!();
        }
      }
    }
  }

  pub(crate) fn banana_sub(&mut self, index: usize) {
    if self.bananas[index].sub() == BananaOverflow::Yes {
      match index {
        0 => {
          self.banana_sub(4);
        }

        2 => {
          self.banana_add(4);
          self.banana_sub(0);
          self.banana_sub(1);
        }

        3 => {
          self.banana_add(4);
          self.banana_sub(0);
          self.banana_sub(2);
        }

        6 => {
          self.banana_add(0);
          self.banana_add(0);
          self.banana_sub(1);
          self.banana_sub(1);
        }

        _ => {
          unreachable!();
        }
      }
    }
  }
}

