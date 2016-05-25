/*
 * Example usage of rpng
 *
 * Simon Whitehead, 2016
 */

extern crate sdl2;
extern crate rpng;

fn main() {

    match rpng::PngFile::from_path(std::env::args().nth(1).unwrap()) {
        Err(error) => println!("Error loading PNG: {:?}", error),
        Ok(png) =>  {
            // A minimum width and height for the Window
            let window_width = std::cmp::max(400, png.w as u32);
            let window_height = std::cmp::max(400, png.h as u32);

            // Setup SDL Window 
            let context = sdl2::init().unwrap();
            let video = context.video().unwrap();
            let mut events = context.event_pump().unwrap();
            let window = video.window("rPNG test window", window_width, window_height)
                .position_centered().opengl()
                .build().unwrap();

            // Instantiate a renderer
            let mut renderer = window.renderer()
            .accelerated()
            .build().unwrap();
            
            // Set the blend mode so that transparent pixels are transparent
            renderer.set_blend_mode(sdl2::render::BlendMode::Blend);

            // Clear the background
            renderer.set_draw_color(sdl2::pixels::Color::RGB(0,0,0));
            renderer.clear();
            
            // Iterate over every pixel in the PNG and plot it within
            // the renderer
            for y in 0..png.h {
                for x in 0..png.w {
                    let p = &png.pixels[png.w * y + x];
                    renderer.set_draw_color(sdl2::pixels::Color::RGBA(p.r, p.g, p.b, p.a));
                    renderer.draw_point(sdl2::rect::Point::new(x as i32, y as i32)).unwrap();
                }
            }

            renderer.present();

            'out:
            loop {
                for event in events.poll_iter() {
                    use sdl2::event::Event::*;

                    match event {
                        KeyDown { keycode, .. } => match keycode { Some(k) => if k == sdl2::keyboard::Keycode::Escape { break 'out }, None => () },
                        Quit { .. } => break 'out,
                        _ => {}
                    }
                }
            }
        }
    }
}
