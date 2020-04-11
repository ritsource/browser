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
    pub fn new(background: [f32; 4], height: f64, width: f64, x: f64, y: f64) -> Self {
        Self {
            gl: GlGraphics::new(OpenGL::V3_2),
            background,
            height,
            width,
            x,
            y,
        }
    }

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
    pub fn new(content: &'a str, color: [f32; 4], background: [f32; 4], font_size: u32, x: f64, y: f64) -> Self {
        Self {
            gl: GlGraphics::new(OpenGL::V3_2),
            content,
            color,
            background,
            font_size,
            x,
            y,
        }
    }
    fn render(&mut self, args: &RenderArgs, glyphs: &mut GlyphCache) {
        use graphics::math::Scalar;
        use graphics::types::{Color, FontSize};
        use graphics::*;

        let x = self.x;
        let y = self.y;
        let content = self.content;

        let color = Color::from(self.color);
        let background = self.background;
        let font_size = FontSize::from(self.font_size);

        let r_height = f64::from(font_size);
        let r_width: f64 = f64::from(font_size * 47 / 100 * self.content.len() as u32);

        let r_x = x;
        let r_y = y - r_height;

        let rect = rectangle::rectangle_by_corners(0.0, 0.0, Scalar::from(r_width), Scalar::from(r_height));

        self.gl.draw(args.viewport(), |c, gl| {
            let tr = c.transform.trans(r_x, r_y);
            let tt = c.transform.trans(x, y);

            rectangle(background, rect, tr, gl);
            text(color, font_size, &content[..], glyphs, tt, gl).unwrap();
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
    const YELLOW: [f32; 4] = [1.0, 1.0, 0.0, 1.0];
    const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
    const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
    const TRANSPARENT: [f32; 4] = [1.0, 1.0, 1.0, 0.0];

    const H1_FONT_SIZE: u32 = 35;
    const P_FONT_SIZE: u32 = 16;

    let mut rect_1 = Rect::new(GREEN, 100.0, 100.0, 0.0, 0.0);
    let mut rect_2 = Rect::new(GREEN, 100.0, 100.0, 120.0, 120.0);

    let mut text_1 = Text::new(
        "Some 1",
        BLACK,
        TRANSPARENT,
        P_FONT_SIZE,
        0.0,
        100.0 + H1_FONT_SIZE as f64,
    );
    let mut text_2 = Text::new(
        "this is p tag under div",
        BLACK,
        TRANSPARENT,
        P_FONT_SIZE,
        0.0,
        100.0 + (H1_FONT_SIZE * 2) as f64 + P_FONT_SIZE as f64,
    );
    let mut text_3 = Text::new(
        "Some 2",
        BLACK,
        TRANSPARENT,
        P_FONT_SIZE,
        0.0,
        100.0 + (H1_FONT_SIZE * 2) as f64 + (P_FONT_SIZE * 2) as f64,
    );
    let mut text_4 = Text::new(
        "another p tag",
        BLACK,
        TRANSPARENT,
        P_FONT_SIZE,
        0.0,
        100.0 + (H1_FONT_SIZE * 3) as f64 + (P_FONT_SIZE * 2) as f64,
    );

    let mut glyphs = GlyphCache::new(
        PathBuf::from("Noto_Serif/NotoSerif-Regular.ttf"),
        (),
        TextureSettings::new(),
    )
    .unwrap();

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            graphics::clear(WHITE, &mut GlGraphics::new(opengl));

            rect_1.render(&args);
            // rect_2.render(&args);
            text_1.render(&args, &mut glyphs);
            text_2.render(&args, &mut glyphs);
            text_3.render(&args, &mut glyphs);
            text_4.render(&args, &mut glyphs);
        }
    }
}
