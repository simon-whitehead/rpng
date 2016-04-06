extern crate sdl2;
extern crate rpng;

use sdl2::{EventPump};
use sdl2::keyboard::Keycode;

fn main() {
    let context = sdl2::init().unwrap();
    let video = context.video().unwrap();
    let mut events = context.event_pump().unwrap();
    let window = video.window("rPNG test window", 800, 600)
        .position_centered().opengl()
        .build().unwrap();

    let renderer = window.renderer()
    .accelerated()
    .build().unwrap();

    match rpng::PngFile::from_path("/Users/Simon/ship.png") {
        Err(error) => println!("Error loading PNG: {:?}", error),
        Ok(_) => println!("Successfully loaded PNG!")
    }

    'out:
    loop {
        for event in events.poll_iter() {
            use sdl2::event::Event::*;

            match event {
                Quit { .. } => break 'out,
                _ => {}
            }
        }
    }
}
