use crate::hex::{Facing, Hex};
use crate::screen::map::MapState;
//use crate::screen::Actor;
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
    pub moving_to: Option<Position>,
    pub speed: u8,
    pub spec: ShipSpec,

    draw_dest: Option<Point2<f32>>,
    draw_rotation: f32,
}

impl Ship {
    pub fn new(ctx: &mut Context, spec_file: &str, position: Position, speed: u8) -> Ship {
        let spec = ShipSpec::new(spec_file);
        let mut path = path::PathBuf::from(IMAGE_PATH);
        path.push(&spec.fx.image);
        let image = graphics::Image::new(ctx, path).unwrap();
        Ship {
            image,
            position,
            moving_to: None,
            speed,
            spec,

            draw_dest: None,
            draw_rotation: 0.0,
        }
    }

    pub fn rotate_to(&mut self, new_facing: Facing) {
        println!("Changing to facing {:?}", new_facing);
        self.moving_to = Some(Position {
            hex: self.position.hex,
            facing: new_facing,
        });
    }

    pub fn move_to(&mut self, new_position: Position) {
        println!("Moving to {:?}", new_position);
        self.moving_to = Some(new_position);
    }

    fn done_moving(&mut self) {
        if self.moving_to.is_some() {
            println!("Destination reached!");
            self.position = self.moving_to.take().unwrap();
        }
    }

    fn set_next_draw_dest(&mut self, map_state: &MapState) {
        let started_at = self.position.hex.to_screen(map_state);
        let moving_to = self.moving_to.as_ref().unwrap();

        let end_dest = moving_to.hex.to_screen(map_state);
        let dx = (end_dest.x - started_at.x) / 60.0;
        let dy = (end_dest.y - started_at.y) / 60.0;
        let v = Vector2::new(dx, dy);

        self.draw_dest.replace(self.draw_dest.unwrap() + v);
        let new_dest = self.draw_dest.unwrap();
        // println!("Current: {:?}  Dest: {:?}", new_dest, end_dest);
        if ulps_eq!(new_dest, end_dest, epsilon = f32::EPSILON, max_ulps = 10_000) {
            self.done_moving();
        }
    }

    fn set_next_draw_rotation(&mut self) {
        let current_degrees = self.draw_rotation.to_degrees();
        let dest_degrees = self.moving_to.as_ref().unwrap().facing.to_degrees();
        // println!("Current facing: {:?} Dest facing: {:?}", current_degrees, dest_degrees);

        // Surely there's a more elegant way to do this, but when looking at the current and
        // destination facing angles, this mess handles turning the shorter distance when crossing
        // 0 degrees.
        if current_degrees == 0.0 && dest_degrees > 180 {
            self.draw_rotation = 358_f32.to_radians();
        } else if dest_degrees == 0 && current_degrees > 357.0 {
            self.draw_rotation = 0_f32.to_radians();
        } else {
            let dr: f32 = if dest_degrees == 0 && current_degrees > 180.0 || dest_degrees as f32 > current_degrees
            {
                2.0
            } else {
                -2.0
            };
            self.draw_rotation += dr.to_radians();
        }
        if (current_degrees - (dest_degrees as f32)).abs() < 3.0 {
            self.draw_rotation = (dest_degrees as f32).to_radians();
            self.position.facing = self.moving_to.as_ref().unwrap().facing;
        }
    }

    pub fn draw(&mut self, ctx: &mut Context, map_state: &MapState) -> GameResult<()> {
        if self.draw_dest.is_none() {
            self.draw_dest = Some(self.position.hex.to_screen(map_state));
            self.draw_rotation = self.position.facing.to_angle();
        } else if self.moving_to.is_some() {
            let moving_to = self.moving_to.as_ref().unwrap();
            if moving_to.facing != self.position.facing {
                self.set_next_draw_rotation();
            } else {
                self.set_next_draw_dest(map_state);
            }
        }

        let scale = (map_state.hex_height - 4.0) / self.image.height() as f32;
        let draw_param = graphics::DrawParam::new()
            .dest(self.draw_dest.unwrap())
            .rotation(self.draw_rotation)
            .offset(Point2::new(0.5, 0.5)) // render from center
            .scale(Vector2::new(scale, scale));
        graphics::draw(ctx, &self.image, draw_param)
    }
}
