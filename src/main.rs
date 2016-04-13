extern crate sdl2;
extern crate rpng;

fn main() {
    let context = sdl2::init().unwrap();
    let video = context.video().unwrap();
    let mut events = context.event_pump().unwrap();
    let window = video.window("rPNG test window", 800, 600)
        .position_centered().opengl()
        .build().unwrap();

    let mut renderer = window.renderer()
    .accelerated()
    .build().unwrap();


    match rpng::PngFile::from_path("/Users/Simon/ship.png") {
        Err(error) => println!("Error loading PNG: {:?}", error),
        Ok(png) =>  {
            let mut texture = renderer.create_texture(sdl2::pixels::PixelFormatEnum::RGBA8888, sdl2::render::TextureAccess::Static, png.w as u32, png.h as u32).unwrap();

            let mut pdata = Vec::new();
            println!("Pixels: {}", png.pixels.len());
            for pixel in &png.pixels {
                pdata.push(pixel.a);
                pdata.push(pixel.b);
                pdata.push(pixel.g);
                pdata.push(pixel.r);
            }

            texture.set_blend_mode(sdl2::render::BlendMode::Blend);

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

                match texture.update(None, &pdata[..], png.pitch) {
                    Err(err) => println!("ERR: {:?}", err),
                    Ok(_) => ()
                }

                renderer.copy(&texture, None, Some(sdl2::rect::Rect::new(0, 0, png.w as u32, png.h as u32)));

                renderer.present();
            }
        }
    }
}
