extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, GlyphCache, OpenGL, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent};
use piston::window::WindowSettings;

use std::path::PathBuf;

#[allow(dead_code)]
pub struct Rect {
    gl: GlGraphics,
    background: [f32; 4],
    height: f64,
    width: f64,
    x: f64,
    y: f64,
}

#[allow(dead_code)]
impl Rect {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::math::Scalar;
        use graphics::*;

        #[allow(unused_mut)]
        let Rect {
            mut background,
            mut height,
            mut width,
            mut x,
            mut y,
            ..
        } = self;

        let rect = rectangle::rectangle_by_corners(0.0, 0.0, Scalar::from(width), Scalar::from(height));

        self.gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform.trans(x, y);

            rectangle(background, rect, transform, gl);
        })
    }
}

#[allow(dead_code)]
pub struct Text<'a> {
    gl: GlGraphics,
    content: &'a str,
    color: [f32; 4],
    background: [f32; 4],
    font_size: u32,
    x: f64,
    y: f64,
}

#[allow(dead_code)]
impl<'a> Text<'a> {
    fn render(&mut self, args: &RenderArgs, glyphs: &mut GlyphCache) {
        use graphics::types::{Color, FontSize};
        use graphics::*;

        let x = self.x;
        let y = self.y;
        let content = self.content;

        let color = Color::from(self.color);
        let font_size = FontSize::from(self.font_size);

        self.gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform.trans(x, y);
            text(color, font_size, &content[..], glyphs, transform, gl).unwrap();
        })
    }
}

#[allow(dead_code)]
pub fn render() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("Browser", [600, 400])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
    const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
    const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
    const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

    let mut rect_1 = Rect {
        gl: GlGraphics::new(opengl),
        background: RED,
        height: 100.0,
        width: 100.0,
        x: 0.0,
        y: 0.0,
    };

    let mut rect_2 = Rect {
        gl: GlGraphics::new(opengl),
        background: GREEN,
        height: 100.0,
        width: 100.0,
        x: 120.0,
        y: 120.0,
    };

    let mut text_1 = Text {
        gl: GlGraphics::new(opengl),
        content: "Hello world!",
        color: BLUE,
        background: GREEN,
        font_size: 30,
        x: 120.0,
        y: 120.0,
    };

    let mut glyphs = GlyphCache::new(
        PathBuf::from("Comic_Neue/ComicNeue-Regular.ttf"),
        (),
        TextureSettings::new(),
    )
    .unwrap();

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            graphics::clear(WHITE, &mut GlGraphics::new(opengl));

            rect_1.render(&args);
            rect_2.render(&args);
            text_1.render(&args, &mut glyphs);
        }
    }
}
