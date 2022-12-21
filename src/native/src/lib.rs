mod utils;

use shipyard::Get;
use std::f64;
use std::str;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use rand::seq::SliceRandom;
use rand::thread_rng;
use shipyard::{Component, EntityId, IntoIter, IntoWithId, View, ViewMut, World};

#[derive(Component)]
struct Health(u32);

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

enum Lover {
    None,
    One(EntityId),
    Many(Vec<EntityId>),
}

#[derive(Component)]
struct Love(Lover);

fn in_acid(
    names: View<Name>,
    positions: View<Position>,
    mut healths: ViewMut<Health>,
    loves: View<Love>,
) {
    for (id, (name, _, mut health)) in (&names, &positions, &mut healths)
        .iter()
        .with_id()
        .filter(|(_, (_, pos, _))| is_in_acid(pos))
    {
        if loves.contains(id) {
            let lover = loves.get(id).unwrap();

            let love_name = match &lover.0 {
                Lover::None => String::from("nobody"),
                Lover::One(id) => names.get(*id).unwrap().0.to_owned(),
                Lover::Many(ids) => ids
                    .iter()
                    .map(|id| names.get(*id).unwrap().0.as_str())
                    .collect::<Vec<&str>>()
                    .join(", "),
            };

            log(format!("I ({}) love {}!", name.0, love_name).as_str());
        } else {
            log(format!("I ({}) have professed neither love nor lack of!", name.0).as_str());
        }

        log(format!(
            "{}:{} is in acid with {} health remaining",
            id.index(),
            name.0,
            health.0
        )
        .as_str());

        if health.0 > 0 {
            health.0 -= 1;
            log("Ouch!");
        } else {
            log("Dead!");
        }
    }
}

fn is_in_acid(_: &Position) -> bool {
    // it's wet season
    true
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct WorldInstance {
    world: shipyard::World,
}

fn random_name() -> String {
    let nouns: Vec<&str> = str::from_utf8(include_bytes!("assets/nouns.txt"))
        .unwrap()
        .split('\n')
        .map(|t| t.trim())
        .collect();

    let adverbs: Vec<&str> = str::from_utf8(include_bytes!("assets/adverbs.txt"))
        .unwrap()
        .split('\n')
        .map(|t| t.trim())
        .collect();

    let adjectives: Vec<&str> = str::from_utf8(include_bytes!("assets/adjectives.txt"))
        .unwrap()
        .split('\n')
        .map(|t| t.trim())
        .collect();

    format!(
        "{}{}{}",
        adverbs.choose(&mut thread_rng()).unwrap(),
        adjectives.choose(&mut thread_rng()).unwrap(),
        nouns.choose(&mut thread_rng()).unwrap()
    )
}

#[wasm_bindgen]
impl WorldInstance {
    pub fn new() -> Result<WorldInstance, JsValue> {
        let mut world = World::new();

        let first = world.add_entity((
            Name(random_name()),
            Position { x: 0.0, y: 0.0 },
            Health(1000),
            Love(Lover::None),
        ));

        let second = world.add_entity((
            Name(random_name()),
            Position { x: 0.0, y: 0.0 },
            Health(5),
            Love(Lover::One(first)),
        ));

        world.add_entity((Name(random_name()), Position { x: 0.0, y: 0.0 }, Health(5)));

        world.add_entity((
            Name(random_name()),
            Position { x: 0.0, y: 0.0 },
            Health(5),
            Love(Lover::Many(vec![first, second])),
        ));

        world
            .borrow::<View<Name>>()
            .unwrap()
            .iter()
            .with_id()
            .for_each(|(_, name)| {
                log(format!("Hello, {}!", name.0).as_str());
            });

        Ok(Self { world })
    }

    pub fn test_render(&self, canvas: web_sys::HtmlCanvasElement) {
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        context.begin_path();

        // Draw the outer circle.
        context
            .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
            .unwrap();

        // Draw the mouth.
        context.move_to(110.0, 75.0);
        context.arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI).unwrap();

        // Draw the left eye.
        context.move_to(65.0, 65.0);
        context
            .arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
            .unwrap();

        // Draw the right eye.
        context.move_to(95.0, 65.0);
        context
            .arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
            .unwrap();

        context.stroke();
    }

    pub fn greet(&self) {
        // self.world
        //     .borrow::<View<Name>>()
        //     .unwrap()
        //     .iter()
        //     .with_id()
        //     // .filter(|(id, name)| id.index() == 1u64)
        //     .for_each(|(_, name)| {
        //         log(format!("Hello, {}!", name.0).as_str());
        //     });

        self.world.run(in_acid);
    }
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
