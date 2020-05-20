mod map;

use super::ship::*;
use ggez::conf::{WindowMode, WindowSetup};
use ggez::*;
use std::env;
use std::path;

const WINDOW_HEIGHT: f32 = 800.0;

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
    let ship = Ship::new(ctx, map_state.hex_height);
    let state = &mut State {
        map_mesh: map_mesh,
        ship: ship,
    };

    event::run(ctx, event_loop, state)
}

struct State {
    map_mesh: graphics::Mesh,
    ship: Ship,
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        graphics::draw(ctx, &self.map_mesh, graphics::DrawParam::default())?;

        let draw_param = graphics::DrawParam::new().scale(self.ship.scale());
        graphics::draw(ctx, self.ship.image(), draw_param)?;

        graphics::present(ctx)?;
        Ok(())
    }
}
