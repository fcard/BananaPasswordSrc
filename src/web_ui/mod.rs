mod util;
use util::*;

use yew::prelude::*;
use super::prelude::*;

fn password_set<'a, T>(
  p: &UseStateHandle<Password>,
  f: &'a impl Fn(Event) -> Option<T>,
  g: impl Fn(&mut Password, T) + 'a) -> impl Fn(Event) + 'a {

  let p = p.clone();
  move |e: Event| {
    if let Some(value) = f(e) {
      let mut p0 = (*p).clone();
      p.set({
        g(&mut p0, value);
        p0
      })
    }
  }
}

#[function_component(App)]
fn app() -> Html {
  let p = use_state(|| Password::new());

  let set_world = password_set(&p, &selected_index, {|p0,index| {
    p0.set_stage(Stage {
      level: p0.stage.level,
      world: index+1
    })
  }});

  let set_level = password_set(&p, &selected_index, {|p0,index| {
    p0.set_stage(Stage {
      level: index+1,
      world: p0.stage.world
    })
  }});

  let set_treasure = password_set(&p, &selected_index, {|p0,index| {
    p0.set_treasure(Treasure::BY_NUMBER[index]);
  }});

  let set_weapon = password_set(&p, &selected_index, {|p0,index| {
    p0.set_weapon(Weapon::BY_NUMBER[index]);
  }});

  macro_rules! weapon_name {
    ($w: ident) => {
      Weapon::select_name::<{Weapon::$w}>().to_string()
    }
  }

  let pwm = match p.stage.world {
    1 => (184, 176),
    2 => (176, 112),
    3 => (168, 48),
    4 => (112, 120),
    5 => (56, 168),
    6 => (56, 104),
    7 => (72, 48),
    _ => (0, 0),
  };

  let plv = match (p.stage.world, p.stage.level) {
    (2, 1 | 3) => (120, 156),
    (2, 2) => (120, 140),
    (3, 2) => (120, 128),
    (3, 3) => (120, 112),
    (4, 1) => (120, 93),
    (4, 2) => (140, 140),
    (4, 3) => (120, 112),
    (5, 1 | 3) => (124, 158),
    (5, 2) => (144, 128),
    (6, 1) => (132, 158),
    (6, 2) => (132, 32),
    (6, 3) => (118, 128),
    (7, 1) => (120, 128),
    (7, 2) => (120, 158),
    (7, 3) => (150, 94),
    _ => (120, 144)
  };

  fn weapon_class(w: Weapon) -> String {
    return Weapon::NAMES[w as usize]
           .to_ascii_lowercase()
           .replace(' ', "-")
           .replace('_', "-");
  }


  html! {
    <div id="main">
      <div id="title">
        <img src="Title.png" alt="BANANA PRINCE"/><br/>
        <h2>{ "PASSWORD GENERATOR" }</h2>
      </div>

      <div id="password">
        {p.bananas.iter().map(|b| html! {
          <span class={format!("banana-{}", *b as u8)}>{*b as u8}</span>
        }).collect::<Html>()}
      </div>
      <div id="chars">
        <img id="lilly" src="Lilly.png"/>
        <img id="prince-yeah" src="PrinceYeah1.png"/>
      </div>
      <br/>

      <div id="options">
        <div id="stage-select">
          <select id="world-select" name="World" onchange={set_world}>
            <option selected=true>{1}</option>
            <option>{2}</option>
            <option>{3}</option>
            <option>{4}</option>
            <option>{5}</option>
            <option>{6}</option>
            <option>{7}</option>
          </select>
          <span id="stage-dash">{"-"}</span>
          <select id="level-select" name="Level" onchange={set_level}>
            <option selected=true>{1}</option>
            <option>{2}</option>
            <option>{3}</option>
          </select>
          <br/>
        </div>
        <div id="stage-view">
          <div id="world-map">
            <img id="prince-wm" src="PrinceFront1.png" alt="PRINCE"
              style={format!("transform: translate({}px, {}px);", pwm.0, pwm.1)}
            />
            <img id="world-map-img" src="WorldMap.png" alt="WORLD MAP"/>
          </div>
          <div id="level-view">
            <img class={format!("stage{}-{}", p.stage.world, p.stage.level)}
                 id="level-view-img" src="1-1-F1.png"/>
            <img id="ui" src="UI1.png"/>
            <img id="prince-lv" alt="PRINCE"
              style={format!("transform: translate({}px, {}px);", plv.0, plv.1)}
            />
            <img id="all-treasures" alt="TREASURES"
              style={format!("width: {}px; ", 16 * p.treasure as usize)}
            />
            <div id="weapon-sprites">
              <img id="weapon1"
                class={format!("weapon-sprite {}", weapon_class(p.weapon))}
                style={format!("--plv-x: {}px; --plv-y: {}px;", plv.0, plv.1)}
              />
              <img id="weapon2"
                class={format!("weapon-sprite {}", weapon_class(p.weapon))}
                style={format!("--plv-x: {}px; --plv-y: {}px;", plv.0, plv.1)}
              />
              <img id="weapon3"
                class={format!("weapon-sprite {}", weapon_class(p.weapon))}
                style={format!("--plv-x: {}px; --plv-y: {}px;", plv.0, plv.1)}
              />
            </div>
            <div id="weapon-name">
              <span>{p.weapon.select_name_rt()}</span>
            </div>
          </div>
        </div>

        <div id="equip-select">
          <select id="treasure-select" name="Treasure" onchange={set_treasure}>
            <option selected=true>{"NONE"}</option>
            <option>{"BANA SWORD"}</option>
            <option>{"BANA SHIELD"}</option>
            <option>{"BANA ARMOR"}</option>
            <option>{"CROWN"}</option>
          </select>

          <select id="weapon-select" name="Weapon" onchange={set_weapon}>
            <option selected=true>{ weapon_name!(StoneAxe) }</option>
            <option>{ weapon_name!(KingAxe) }</option>
            <option>{ weapon_name!(StarAxe) }</option>
            <option>{ weapon_name!(Magic) }</option>
            <option>{ weapon_name!(Capsule) }</option>
            <option>{ weapon_name!(Maxim) }</option>
            <option>{ weapon_name!(Meteor) }</option>
            <option>{ weapon_name!(Megalo) }</option>
            <option>{ weapon_name!(Fold) }</option>
            <option>{ weapon_name!(CrossAxe) }</option>
            <option>{ weapon_name!(GrandAxe) }</option>
            <option>{ weapon_name!(CurseAxe) }</option>
            <option>{ weapon_name!(Mercury) }</option>
            <option>{ weapon_name!(Flash) }</option>
            <option>{ weapon_name!(Crusher) }</option>
            <option>{ weapon_name!(KingShip) }</option>
          </select>
        </div>
      </div>
    </div>
  }
}

pub(crate) fn web_ui() {
  yew::Renderer::<App>::new().render();
}

