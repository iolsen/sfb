mod imgui_wrapper;
pub mod main_menu;
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

pub const MENU_HEIGHT: f32 = 20.0;
pub const WINDOW_HEIGHT: f32 = MENU_HEIGHT + 800.0; // Laptop
//const WINDOW_HEIGHT: f32 = MENU_HEIGHT + 1300.0; // Desktop

struct GameState {
    imgui_wrapper: ImGuiWrapper,
    hidpi_factor: f32,
    mouse_down: bool,
    map_state: MapState,
    map_mesh: graphics::Mesh,
    ships: Vec<Box<Ship>>,
    // actors: Vec<Box<dyn Actor>>,
}

// A thing that can be drawn on the map.
//pub trait Actor {
//    fn draw(&mut self, ctx: &mut Context, map_state: &MapState) -> GameResult<()>;
//    fn move_to(&mut self, new_position: Position);
//}

pub fn run() -> GameResult<()> {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };
    let map_state = map::init(Point2::new(0.0, MENU_HEIGHT), WINDOW_HEIGHT - MENU_HEIGHT);

    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("sfbv1", "ian_olsen")
        .add_resource_path(resource_dir)
        .window_setup(WindowSetup::default().title("Star Fleet Battles Volume 1"))
        .window_mode(
            WindowMode::default()
                .dimensions(map_state.width, WINDOW_HEIGHT)
                .resizable(true),
        )
        .build()?;

    let hidpi_factor = ggez::graphics::window(&ctx).get_hidpi_factor() as f32;
    println!("hidpi_factor = {}", hidpi_factor);

    let map_mesh = map::build_mesh(ctx, &map_state)?;

    // "The Duel"
    let ca: Box<Ship> = Box::new(Ship::new(
        ctx,
        "federation/ca.toml",
        Position {
            hex: Hex::new(6, 29).unwrap(),
            facing: Facing::A,
        },
        15,
    ));
    let d7: Box<Ship> = Box::new(Ship::new(
        ctx,
        "klingon/d7.toml",
        Position {
            hex: Hex::new(41, 2).unwrap(),
            facing: Facing::E,
        },
        15,
    ));
    let ships = vec![ca, d7];

    let state = &mut GameState {
        imgui_wrapper: ImGuiWrapper::new(ctx),
        hidpi_factor,
        mouse_down: false,
        map_state,
        map_mesh,
        ships,
    };

    event::run(ctx, event_loop, state)
}

impl ggez::event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        graphics::draw(ctx, &self.map_mesh, graphics::DrawParam::default())?;

        for ship in self.ships.iter_mut() {
            ship.draw(ctx, &self.map_state)?;
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
        let hex = Hex::from_screen(p, &self.map_state);
        println!("Mouse button pressed: {:?}, in hex {:?}", button, hex);
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        self.imgui_wrapper.update_mouse_down((false, false, false));
        self.mouse_down = false;
        let p = Point2::new(x, y);
        let hex = Hex::from_screen(p, &self.map_state);
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
            KeyCode::E => {
                let d7 = &mut self.ships[1];
                let pos = Position {
                    facing: d7.position.facing.turn_right(),
                    hex: d7.position.hex,
                };
                d7.move_to(pos);
            }
            KeyCode::P => {
                self.imgui_wrapper.open_popup();
            }
            KeyCode::Q => {
                if input::keyboard::is_mod_active(ctx, input::keyboard::KeyMods::LOGO) {
                    println!("cmd-q: quitting");
                    event::quit(ctx);
                } else {
                    let d7 = &mut self.ships[1];
                    let pos = Position {
                        facing: d7.position.facing.turn_left(),
                        hex: d7.position.hex,
                    };
                    d7.move_to(pos);
                }
            }
            KeyCode::W => {
                let d7 = &mut self.ships[1];
                let dest = d7.position.hex.neighbor(d7.position.facing);
                if dest.is_some() {
                    let pos = Position {
                        facing: d7.position.facing,
                        hex: dest.unwrap(),
                    };
                    d7.move_to(pos);
                }
            }
            _ => (),
        }
    }

    fn resize_event(&mut self, ctx: &mut Context, _width: f32, height: f32) {
        // TODO DRY this when startup in run() is better.
        self.hidpi_factor = ggez::graphics::window(&ctx).get_hidpi_factor() as f32;
        println!("hidpi_factor = {}", self.hidpi_factor);
        self.map_state = map::init(Point2::new(0.0, MENU_HEIGHT), height - MENU_HEIGHT);
        self.map_mesh = map::build_mesh(ctx, &self.map_state).unwrap();

        for ship in self.ships.iter_mut() {
            ship.invalidate();
        }
    }
}
