extern crate cgmath;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

pub mod view;
pub mod model;
pub mod controller;

use cgmath::*;
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL, GlyphCache, TextureSettings };
use graphics::*;


pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    current_mouse_pos: Vector2<f64>,
}

impl App {
    // fn render<C, E>(&mut self, args: &RenderArgs, glyphs: &mut C) 
    // where E: Debug, C: CharacterCache<Texture=Texture, Error=E> {

    // }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl_version = OpenGL::V3_2;

    let width = 800;
    let height = 800;
    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "Garden Opus",
            [width, height]
        )
        .opengl(opengl_version)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let ref mut glyphs = GlyphCache::new(
        "assets/FiraSans-Regular.ttf", 
        (), 
        TextureSettings::new()
    ).expect("Could not load font");


    let center = Vector2::new(width as f64/2.0, height as f64/2.0);

    let board = model::board::Board::empty();
    let board_controller = controller::BoardController::new(board);
    let board_view = view::BoardView::new();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl_version),
        current_mouse_pos: center,
    };


    let mut events = Events::new(EventSettings::new().lazy(true));
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

            app.gl.draw(r.viewport(), |c, gl| {
                clear(GREEN, gl);
                board_view.draw(&board_controller, &c, gl, glyphs);
            });
        }

        if let Some(xy) = e.mouse_cursor_args() {
            app.current_mouse_pos = Vector2::from(xy);
        }
    }
}
