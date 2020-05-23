mod imgui_wrapper;
pub mod map;

use crate::hex::{Facing, Hex};
use crate::ship::{Position, Ship};
use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{KeyCode, KeyMods, MouseButton};
use ggez::nalgebra::Point2;
use ggez::*;
use imgui_wrapper::ImGuiWrapper;
use map::MapState;
use std::env;
use std::path;

//const WINDOW_HEIGHT: f32 = 800.0; // Laptop
const WINDOW_HEIGHT: f32 = 1300.0; // Desktop

struct GameState {
    imgui_wrapper: ImGuiWrapper,
    hidpi_factor: f32,
    mouse_down: bool,
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

    // Seems to not work. Should be 2.0 on retina, 1.0 on external.
    // let hidpi_factor = event_loop.get_primary_monitor().get_hidpi_factor() as f32;
    let hidpi_factor = 2.0;
    println!("hidpi_factor = {}", hidpi_factor);

    let map_mesh = map::build_mesh(ctx, &map_state)?;

    // "The Duel"
    let ca: Box<dyn Actor> = Box::new(Ship::new(
        ctx,
        "federation/ca.toml",
        Position {
            hex: Hex::new(6, 29).unwrap(),
            facing: Facing::A,
        },
        15,
    ));
    let d7: Box<dyn Actor> = Box::new(Ship::new(
        ctx,
        "klingon/d7.toml",
        Position {
            hex: Hex::new(41, 2).unwrap(),
            facing: Facing::E,
        },
        15,
    ));
    let actors = vec![ca, d7];

    let state = &mut GameState {
        imgui_wrapper: ImGuiWrapper::new(ctx),
        hidpi_factor,
        mouse_down: false,
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

        self.imgui_wrapper.render(ctx, self.hidpi_factor);

        graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        self.imgui_wrapper.update_mouse_pos(x, y);
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        self.imgui_wrapper.update_mouse_down((
            button == MouseButton::Left,
            button == MouseButton::Right,
            button == MouseButton::Middle,
        ));
        self.mouse_down = true;
        let p = Point2::new(x, y);
        let hex = Hex::from_screen(p, self.map_state.hex_edge);
        println!("Mouse button pressed: {:?}, in hex {:?}", button, hex);
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        self.imgui_wrapper.update_mouse_down((false, false, false));
        self.mouse_down = false;
        let p = Point2::new(x, y);
        let hex = Hex::from_screen(p, self.map_state.hex_edge);
        println!("Mouse button released: {:?}, in hex {:?}", button, hex);
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        keymods: KeyMods,
        _repeat: bool,
    ) {
        if keymods == input::keyboard::KeyMods::empty() {
            println!("Key down: {:?}", keycode);
        } else {
            println!("Key down: {:?}-{:?}", keymods, keycode);
        }
        match keycode {
            KeyCode::P => {
                self.imgui_wrapper.open_popup();
            }
            KeyCode::Q => {
                if input::keyboard::is_mod_active(ctx, input::keyboard::KeyMods::LOGO) {
                    event::quit(ctx)
                }
            }
            _ => (),
        }
    }
}
