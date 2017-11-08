extern crate graphics;
extern crate glium_graphics;
extern crate piston;

use glium_graphics::{
    Flip, Glium2d, GliumWindow, GlyphCache, OpenGL, Texture, TextureSettings
};
use piston::input::*;
use piston::event_loop::EventLoop;
use piston::window;
use piston::window::*;
use piston::window::WindowSettings;
use graphics::draw_state::Blend;
use std::path::Path;

use graphics::character::*;

fn main() {
    println!("Hello, world!");

    let opengl = OpenGL::V3_2;

    let (w, h) = (640, 480);

    let ref mut window: GliumWindow =
        WindowSettings::new("Sudoku", [w, h])
        .exit_on_esc(true).opengl(opengl).build().unwrap();

    let mut g2d = Glium2d::new(opengl, window);

    let grey = [0.8, 0.8, 0.8, 1.0];

    window.set_lazy(true);
    while let Some(e) = window.next() {
        if let Some(args) = e.render_args() {
            use graphics::*;

            let mut target = window.draw();
            g2d.draw(&mut target, args.viewport(), |c, g| {
                clear(grey, g); // Grey background
                // Do all other rendering
            });
            target.finish().unwrap();
        }

        if let Some(arg) = e.press_args() {
            if let Button::Keyboard(key) = arg {
                match key {
                    Key::A => {
                        println!("A");
                    }
                    _ => {}
                }
            }
        }
    }
            
}
