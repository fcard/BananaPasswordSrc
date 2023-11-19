use yew::Event;
use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;

pub(super) fn event_target<T: JsCast>(e: Event) -> Option<T> {
  if let Some(t) = e.target() &&
     let Ok(inp) = t.dyn_into::<T>() {
    return Some(inp);
  } else {
    return None;
  }
}

pub(super) fn selected_index(e: Event) -> Option<usize> {
  event_target::<HtmlSelectElement>(e)
   .and_then(|i| Some(i.selected_index() as usize))
}

