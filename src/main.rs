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


    match rpng::PngFile::from_path(std::env::args().nth(1).unwrap()) {
        Err(error) => println!("Error loading PNG: {:?}", error),
        Ok(png) =>  {
            let mut texture = renderer.create_texture(sdl2::pixels::PixelFormatEnum::RGBA8888, sdl2::render::TextureAccess::Static, png.w as u32, png.h as u32).unwrap();

            renderer.set_blend_mode(sdl2::render::BlendMode::Blend);

            let mut pdata = Vec::new();
            for pixel in &png.pixels {
                pdata.push(pixel.a);
                pdata.push(pixel.b);
                pdata.push(pixel.g);
                pdata.push(pixel.r);
            }

            match texture.update(None, &pdata[..], png.pitch) {
                Err(err) => println!("ERR: {:?}", err),
                Ok(_) => ()
            }

            renderer.set_draw_color(sdl2::pixels::Color::RGB(0,0,0));
            renderer.clear();

            renderer.copy(&texture, None, Some(sdl2::rect::Rect::new(0, 0, png.w as u32, png.h as u32)));
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
                
                /*for y in 0..png.h {
                    for x in 0..png.w {
                        let p = &png.pixels[png.w * y + x];
                        renderer.set_draw_color(sdl2::pixels::Color::RGB(p.r, p.g, p.b));
                        renderer.draw_point(sdl2::rect::Point::new(x as i32, y as i32));
                    }
                }*/

//                renderer.present();
            }
        }
    }
}
