extern crate sdl2;
extern crate rpng;

fn main() {
    let context = sdl2::init().unwrap();
    let video = context.video().unwrap();
    let mut events = context.event_pump().unwrap();
    let window = video.window("rPNG test window", 1440, 900)
        .position_centered().opengl()
        .build().unwrap();

    let mut renderer = window.renderer()
    .accelerated()
    .build().unwrap();


    match rpng::PngFile::from_path("/Users/Simon/vslogo.png") {
        Err(error) => println!("Error loading PNG: {:?}", error),
        Ok(png) =>  {
            let mut texture = renderer.create_texture(sdl2::pixels::PixelFormatEnum::RGB888, sdl2::render::TextureAccess::Static, png.w as u32, png.h as u32).unwrap();

            println!("Width: {}, Height: {}, Pixels: {}", png.w, png.h, png.pixels.len());

            renderer.set_blend_mode(sdl2::render::BlendMode::Blend);

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

                renderer.set_draw_color(sdl2::pixels::Color::RGB(0,0,0));
                renderer.clear();
                
                for y in 0..png.h {
                    for x in 0..png.w {
                        let p = &png.pixels[y * x + x];
                        renderer.set_draw_color(sdl2::pixels::Color::RGB(p.r, p.g, p.b));
                        renderer.draw_point(sdl2::rect::Point::new(x as i32, y as i32));
                    }
                }

                renderer.present();
            }
        }
    }
}
