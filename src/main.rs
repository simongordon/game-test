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

type Pos = f64;

struct GameObject {
    pub x: Pos,
    pub y: Pos,
    pub size: f64,
    pub colour: [f32; 4],
    pub solid: bool,
}

impl GameObject {
    pub fn hedge(x: Pos, y: Pos) -> GameObject {
        let dark_green = [0.0, 150.0/255.0, 0.0, 1.0];
        GameObject {
            x,
            y,
            size: 40.0,
            colour: dark_green,
            solid: true,
        }
    }
}

fn main() {
    println!("Hello, world!");

    let opengl = OpenGL::V3_2;

    let (w, h) = (640, 480);

    let ref mut window: GliumWindow =
        WindowSettings::new("Sudoku", [w, h])
        .exit_on_esc(true).opengl(opengl).build().unwrap();

    let mut g2d = Glium2d::new(opengl, window);

    let white = [1.0, 1.0, 1.0, 1.0];
    let grey = [0.8, 0.8, 0.8, 1.0];
    let red = [1.0, 0.0, 0.0, 1.0];

    let mut player = GameObject {
        x: 50.0,
        y: 50.0,
        size: 40.0,
        colour: red,
        solid: true,
    };

    let environment = [
        GameObject::hedge(227.0, 265.0),
        GameObject::hedge(290.0, 265.0),
    ];


    window.set_lazy(true);
    while let Some(e) = window.next() {
        if let Some(args) = e.render_args() {
            use graphics::*;

            let mut target = window.draw();
            g2d.draw(&mut target, args.viewport(), |c, g| {
                clear(grey, g); // Grey background
                let GameObject {
                    x,
                    y,
                    size,
                    colour,
                    ..
                } = player;
                Rectangle::new(colour).draw([x, y, size, size], &c.draw_state, c.transform, g);
                for obj in environment.iter() {
                    let &GameObject {
                        x,
                        y,
                        size,
                        colour,
                        ..
                    } = obj;
                    Rectangle::new(colour).draw([x, y, size, size], &c.draw_state, c.transform, g);
                }
            });
            target.finish().unwrap();
        }

        if let Some(arg) = e.press_args() {
            if let Button::Keyboard(key) = arg {
                let move_amount = 5.0;
                match key {
                    Key::A => {
                        println!("A");
                    }
                    Key::K | Key::Up => {
                        player.y -= move_amount;
                    }
                    Key::J | Key::Down => {
                        player.y += move_amount;
                    }
                    Key::H | Key::Left => {
                        player.x -= move_amount;
                    }
                    Key::L | Key::Right => {
                        player.x += move_amount;
                    }
                    _ => {}
                }
            }
        }
    }
            
}
