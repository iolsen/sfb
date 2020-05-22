use crate::hex::{Facing, Hex};
use crate::screen::Actor;
use crate::screen::map::MapState;
use crate::ship_spec::*;
use ggez::graphics;
use ggez::nalgebra::{Point2, Vector2};
use ggez::{Context, GameResult};
use std::path;

const IMAGE_PATH: &str = "/gfx/ships";

#[derive(PartialEq, Eq, Debug)]
pub struct Position {
    pub hex: Hex,
    pub facing: Facing,
}

pub struct Ship {
    pub image: graphics::Image,
    pub position: Position,
}

impl Ship {
    pub fn new(ctx: &mut Context, position: Position, spec_file : &str) -> Ship {
        let spec = ShipSpec::new(spec_file);
        let mut path = path::PathBuf::from(IMAGE_PATH);
        path.push(spec.fx.image);
        let image = graphics::Image::new(ctx, path).unwrap();
        Ship {
            image,
            position,
        }
    }
}

impl Actor for Ship {
    fn draw(&self, ctx: &mut Context, map_state: &MapState) -> GameResult<()> {
        let screen = self.position.hex.to_screen(map_state.hex_edge);
        let scale = (map_state.hex_height - 4.0) / self.image.height() as f32;
        let draw_param = graphics::DrawParam::new()
            .dest(self.position.hex.to_screen(map_state.hex_edge))
            .rotation(self.position.facing.to_angle())
            .offset(Point2::new(0.5, 0.5)) // rotate from center
            .scale(Vector2::new(scale, scale));
        graphics::draw(ctx, &self.image, draw_param)
    }
}
