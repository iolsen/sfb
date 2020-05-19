use ggez::conf::{WindowMode, WindowSetup};
use ggez::nalgebra::{Point2, Vector2};
use ggez::*;

const WINDOW_HEIGHT: f32 = 1000.0;

struct Sizes {
    start_point: Point2<f32>,
    window_width: f32,
    window_height: f32,
    hex_height: f32,
    hex_edge: f32,
    vector: Vector2<f32>,
}
fn get_sizes(window_height: f32) -> Sizes {
    let hex_height = window_height / 30.5;
    let hex_edge = hex_height / 3_f32.sqrt();
    let start_point = Point2::new(hex_edge, hex_height * 0.5);
    let vector = Vector2::new(hex_edge * 2.0 * 0.75, hex_height * 0.5);

    let hex_width = 2.0 * hex_edge;
    let window_width = 0.75 * hex_width * 60.0 + (hex_width * 0.25);

    Sizes {
        start_point,
        window_width,
        window_height,
        hex_height,
        hex_edge,
        vector,
    }
}

struct State {
    map_mesh: graphics::Mesh,
}

fn hex_vertex(center: Point2<f32>, size: f32, i: usize) -> Point2<f32> {
    let angle_deg = (60 * i) as f32;
    let angle_rad = std::f32::consts::PI / 180.0 * angle_deg;
    Point2::new(
        center.x + size * angle_rad.cos(),
        center.y + size * angle_rad.sin(),
    )
}

fn hex_points(center: Point2<f32>, size: f32) -> [Point2<f32>; 6] {
    [
        hex_vertex(center, size, 0),
        hex_vertex(center, size, 1),
        hex_vertex(center, size, 2),
        hex_vertex(center, size, 3),
        hex_vertex(center, size, 4),
        hex_vertex(center, size, 5),
    ]
}

fn build_hex_map_mesh(ctx: &mut Context, sizes: &Sizes) -> GameResult<graphics::Mesh> {
    let builder = &mut graphics::MeshBuilder::new();

    for col in 0..60 {
        let x = sizes.start_point.x + sizes.vector.x * col as f32;
        for row in 0..30 {
            let y = if col % 2 == 0 {
                // even row
                sizes.start_point.y + sizes.hex_height * row as f32
            } else {
                // odd row
                sizes.start_point.y + sizes.vector.y + sizes.hex_height * row as f32
            };
            let center = Point2::new(x, y);
            let points = hex_points(center, sizes.hex_edge);
            builder.polygon(graphics::DrawMode::stroke(1.0), &points, graphics::WHITE)?;
        }
    }

    builder.build(ctx)
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

fn main() {
    let sizes = get_sizes(WINDOW_HEIGHT);
    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("sfbv1", "ian_olsen")
        .window_setup(WindowSetup::default().title("Star Fleet Battles Volume 1"))
        .window_mode(
            WindowMode::default()
                .dimensions(sizes.window_width, sizes.window_height)
                .resizable(false),
        )
        .build()
        .unwrap();

    let map_mesh = build_hex_map_mesh(ctx, &sizes).unwrap();
    let state = &mut State { map_mesh: map_mesh };

    event::run(ctx, event_loop, state).unwrap();
}
