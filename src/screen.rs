pub mod map;

use crate::hex::{Facing, Hex};
use crate::ship::{Position, Ship};
use ggez::conf::{WindowMode, WindowSetup};
use ggez::*;
use map::MapState;
use std::env;
use std::path;

const WINDOW_HEIGHT: f32 = 800.0; // Laptop
//const WINDOW_HEIGHT: f32 = 1300.0; // Desktop

struct GameState {
    map_state: MapState,
    map_mesh: graphics::Mesh,
    actors: Vec<Box<dyn Actor>>,
}

// A thing that can be drawn on the map.
pub trait Actor {
    fn draw(&self, ctx: &mut Context, map_state: &MapState) -> GameResult<()>;
}

pub fn run() -> GameResult<()> {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };
    let map_state = map::init(WINDOW_HEIGHT);
    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("sfbv1", "ian_olsen")
        .add_resource_path(resource_dir)
        .window_setup(WindowSetup::default().title("Star Fleet Battles Volume 1"))
        .window_mode(
            WindowMode::default()
                .dimensions(map_state.width, map_state.height)
                .resizable(false),
        )
        .build()?;

    let map_mesh = map::build_mesh(ctx, &map_state)?;
    let ca: Box<dyn Actor> = Box::new(
        Ship::new(ctx,
                  Position { hex: Hex::new(6, 29).unwrap(), facing: Facing::A },
                  "federation/ca.toml"));
    let d7: Box<dyn Actor> = Box::new(
        Ship::new(ctx,
                  Position { hex: Hex::new(41, 2).unwrap(), facing: Facing::E },
                  "klingon/d7.toml"));
    let actors = vec![ca, d7];
    let state = &mut GameState {
        map_state,
        map_mesh,
        actors,
    };

    event::run(ctx, event_loop, state)
}

impl ggez::event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        graphics::draw(ctx, &self.map_mesh, graphics::DrawParam::default())?;

        for actor in self.actors.iter() {
            actor.draw(ctx, &self.map_state)?;
        }

        graphics::present(ctx)?;
        Ok(())
    }
}
