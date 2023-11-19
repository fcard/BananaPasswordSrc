#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) enum Banana {
  Unpeeled = 0,
  HalfPeeled = 1,
  Peeled = 2,
  Eaten = 3
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) enum BananaOverflow {
  Yes,
  No
}

impl Banana {
  pub(crate) fn add(&mut self) -> BananaOverflow {
    if let Some(b) = self.next_no_overflow() {
      *self = b;
      return BananaOverflow::No;
    } else {
      *self = Banana::Unpeeled;
      return BananaOverflow::Yes;
    }
  }

  pub(crate) fn sub(&mut self) -> BananaOverflow {
    if let Some(b) = self.prev_no_overflow() {
      *self = b;
      return BananaOverflow::No;
    } else {
      *self = Banana::Eaten;
      return BananaOverflow::Yes;
    }
  }


  fn next_no_overflow(self) -> Option<Banana> {
    use Banana::*;
    match self {
      Unpeeled   => Some(HalfPeeled),
      HalfPeeled => Some(Peeled),
      Peeled     => Some(Eaten),
      Eaten      => None
    }
  }

  fn prev_no_overflow(self) -> Option<Banana> {
    use Banana::*;
    match self {
      Unpeeled   => None,
      HalfPeeled => Some(Unpeeled),
      Peeled     => Some(HalfPeeled),
      Eaten      => Some(Peeled)
    }
  }
}

