use ggez::graphics::{DrawMode, Mesh, MeshBuilder, WHITE};
use ggez::nalgebra::{Point2, Vector2};
use ggez::{Context, GameResult};

pub struct MapState {
    pub origin: Point2<f32>, // The screen coordinate of the map's upper left corner
    pub width: f32,
    pub height: f32,
    pub start_point: Point2<f32>, // The screen coordinate for the center of hex 0,0
    pub hex_height: f32,
    pub hex_edge: f32,
    vector: Vector2<f32>,
}

pub fn init(origin: Point2<f32>, height: f32) -> MapState {
    let hex_height = height / 30.5;
    let hex_edge = hex_height / 3_f32.sqrt();
    let start_point = Point2::new(hex_edge + origin.x, hex_height * 0.5 + origin.y);
    let vector = Vector2::new(hex_edge * 2.0 * 0.75, hex_height * 0.5);

    let hex_width = 2.0 * hex_edge;
    let width = 0.75 * hex_width * 60.0 + (hex_width * 0.25);

    MapState {
        origin,
        width,
        height,
        start_point,
        hex_height,
        hex_edge,
        vector,
    }
}

pub fn build_mesh(ctx: &mut Context, map_state: &MapState) -> GameResult<Mesh> {
    let builder = &mut MeshBuilder::new();

    for col in 0..60 {
        let x = map_state.start_point.x + map_state.vector.x * col as f32;
        for row in 0..30 {
            let y = if col % 2 == 0 {
                // even row
                map_state.start_point.y + map_state.hex_height * row as f32
            } else {
                // odd row
                map_state.start_point.y + map_state.vector.y + map_state.hex_height * row as f32
            };
            let center = Point2::new(x, y);
            let points = hex_points(center, map_state.hex_edge);
            builder.polygon(DrawMode::stroke(1.0), &points, WHITE)?;
        }
    }

    builder.build(ctx)
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
