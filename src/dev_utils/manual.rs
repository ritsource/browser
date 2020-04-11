extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

#[allow(dead_code)]
pub struct Rect {
    gl: GlGraphics,
}

#[allow(dead_code)]
impl Rect {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let rect = rectangle::rectangle_by_corners(0.0, 0.0, 100.0, 100.0);
        let x: f64 = 0.0;
        let y: f64 = 0.0;

        self.gl.draw(args.viewport(), |c, gl| {
            clear(WHITE, gl);

            let transform = c.transform.trans(x, y);

            rectangle(RED, rect, transform, gl);
        })
    }
}

#[allow(dead_code)]
pub fn render() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [600, 400])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut rect = Rect {
        gl: GlGraphics::new(opengl),
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            rect.render(&args);
        }
    }
}
