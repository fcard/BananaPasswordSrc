use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Stage {
  pub(crate) level: usize,
  pub(crate) world: usize
}

impl Ord for Stage {
  fn cmp(&self, other: &Self) -> Ordering {
    unsafe { self.partial_cmp(&other).unwrap_unchecked() }
  }
}

impl PartialOrd for Stage {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    match self.world.cmp(&other.world) {
      Ordering::Equal => {
        return Some(self.level.cmp(&other.level));
      }

      unequal => {
        return Some(unequal);
      }
    }
  }
}

impl Stage {
  pub(crate) const START: Self = Stage {
    level: 1,
    world: 1
  };

  pub(crate) const END: Self = Stage {
    level: 7,
    world: 3
  };

  pub(crate) fn next(&mut self) {
    if self.level == 3 {
      if self.world < 7 {
        self.level = 1;
        self.world += 1;
      }
    } else {
      self.level += 1;
    }
  }

  pub(crate) fn prev(&mut self) {
    if self.level == 1 {
      if self.world > 1 {
        self.level = 3;
        self.world -= 1;
      }
    } else {
      self.level -= 1;
    }
  }
}


