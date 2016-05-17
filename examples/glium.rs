extern crate glium;
extern crate nuklear_rs as nk;

use glium::{DisplayBuild, Surface};

fn main() {
    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(800, 600)
        .with_title(format!("Nuklear Widgets Example"))
        .build_glium()
        .unwrap();

    loop {
        for e in display.poll_events() {
            match e {
                glium::glutin::Event::Closed => break,
                _ => {}
            }
        }

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.finish().unwrap();
    }
}
