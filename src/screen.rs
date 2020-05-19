mod map;

use ggez::conf::{WindowMode, WindowSetup};
use ggez::*;

const WINDOW_HEIGHT: f32 = 800.0;

pub fn run() -> GameResult<()> {
    let map_state = map::init(WINDOW_HEIGHT);
    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("sfbv1", "ian_olsen")
        .window_setup(WindowSetup::default().title("Star Fleet Battles Volume 1"))
        .window_mode(
            WindowMode::default()
                .dimensions(map_state.width, map_state.height)
                .resizable(false),
        )
        .build()?;

    let map_mesh = map::build_mesh(ctx, &map_state)?;
    let state = &mut State { map_mesh: map_mesh };

    event::run(ctx, event_loop, state)
}

struct State {
    map_mesh: graphics::Mesh,
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        graphics::draw(ctx, &self.map_mesh, graphics::DrawParam::default())?;
        graphics::present(ctx)?;
        Ok(())
    }
}
