extern crate glium;
extern crate nuklear_rs as nk;

use glium::{DisplayBuild, Surface};
use nk::Color;

fn main() {
    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(800, 600)
        .with_title(format!("Nuklear Widgets Example"))
        .build_glium()
        .unwrap();

    let bg_color = Color::rgb(0, 0, 255);

    loop {
        for e in display.poll_events() {
            match e {
                glium::glutin::Event::Closed => break,
                _ => {}
            }
        }

        let mut target = display.draw();
        let col_slice: [f32; 4] = bg_color.into();
        target.clear_color(col_slice[0], col_slice[1], col_slice[2], col_slice[3]);
        target.finish().unwrap();
    }
}
