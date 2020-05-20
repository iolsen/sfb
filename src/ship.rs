use ggez::Context;
use ggez::graphics;
use ggez::nalgebra::Vector2;

pub struct Ship {
    image: graphics::Image,
    scale: Vector2<f32>,
    _private: (),
}

impl Ship {
    pub fn new(ctx: &mut Context, hex_height: f32) -> Ship {
        let ship_image = graphics::Image::new(ctx, "/gfx/ships/heavycruiser_enterprise.png").unwrap();
        let scale = (hex_height - 5.0) / ship_image.height() as f32;
        Ship {
            image: ship_image,
            scale: Vector2::new(scale, scale),
            _private: (),
        }
    }

    pub fn image(&self) -> &graphics::Image {
        &self.image
    }

    pub fn scale(&self) -> Vector2<f32> {
        self.scale
    }
}
