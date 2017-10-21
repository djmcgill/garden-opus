extern crate cgmath;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use cgmath::*;
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    current_mouse_pos: Vector2<f64>,
}

impl App {
        fn render(&mut self, args: &RenderArgs) {
            use graphics::*;

            const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
            const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

            let square = rectangle::square(0.0, 0.0, 50.0);

            self.gl.draw(args.viewport(), |c, gl| {
                // Clear the screen.
                clear(GREEN, gl);
                let transform = c.transform.trans(0.0, 0.0)
                                        .rot_rad(0.0)
                                        .trans(-25.0, -25.0);

                // Draw a box rotating around the middle of the screen.
                rectangle(RED, square, transform, gl);
            });
        }

        fn update(&mut self, args: &UpdateArgs) {
            let _dt = args.dt;
        }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    let width = 800;
    let height = 800;
    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "spinning-square",
            [width, height]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let center = Vector2::new(width as f64/2.0, height as f64/2.0);

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        current_mouse_pos: center,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }

        if let Some(xy) = e.mouse_cursor_args() {
            app.current_mouse_pos = Vector2::from(xy);
        }
    }
}
