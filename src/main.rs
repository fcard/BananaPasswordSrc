#![allow(incomplete_features)]
#![feature(byte_slice_trim_ascii)]
#![feature(let_chains)]
#![feature(step_trait)]
#![feature(proc_macro_hygiene)]
#![feature(generic_const_exprs)]
#![feature(adt_const_params)]

use attrs::*;

mod banana;
mod stage;
mod treasure;
mod weapon;
mod weapon_name;
mod password;

pub(crate) mod prelude {
  pub use attrs::*;
  #[cli] pub(crate) use super::banana::*;
  pub(crate) use super::stage::*;
  pub(crate) use super::treasure::*;
  pub(crate) use super::weapon::*;
  pub(crate) use super::password::*;
}

#[cli]
mod cli;

#[web]
mod web_ui;



/*
#[cfg(target_family = "wasm")]
#[function_component(Password)]
fn app(props: &AppProps) -> Html {
  props.p.set_stage(props.s);
  html!{
    <div>
      {props.p.with_bananas({|b| format!("{:?}", b)})}
    </div>
  }
}
*/


#[cfg(target_family = "wasm")]
fn main() { web_ui::web_ui(); }

#[cfg(not(target_family = "wasm"))]
fn main() { cli::cli(); }


