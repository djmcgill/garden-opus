extern crate cgmath;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

pub mod view;
pub mod model;
pub mod controller;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL, GlyphCache, TextureSettings };
use graphics::*;

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


    let mut board = model::GameState::new();
    board.selected_tile = Some((1, 3));
    let mut board_controller = controller::BoardController::new(board);
    let board_view = view::BoardView::new();
    let mut gl = GlGraphics::new(opengl_version);

    let mut events = Events::new(EventSettings::new().lazy(true));
    while let Some(e) = events.next(&mut window) {
        board_controller.event(&e);
        if let Some(args) = e.render_args() {
            const GREY: [f32; 4] = [56.0/255.0, 89.0/255.0, 82.0/255.0, 1.0];

            gl.draw(args.viewport(), |c, gl| {
                clear(GREY, gl);
                board_view.draw(&board_controller, &c, gl, glyphs);
            });
        }

        // if let Some(xy) = e.mouse_cursor_args() {
        //     app.current_mouse_pos = Vector2::from(xy);
        // }
    }
}
