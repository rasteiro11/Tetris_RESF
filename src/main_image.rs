use std::time::Duration;

use sdl2::{
    event::Event,
    image::{LoadTexture, INIT_JPG, INIT_PNG},
    keyboard::Keycode,
    pixels::Color,
    render::TextureCreator,
};

pub fn main() {
    let sdl_context = sdl2::init().expect("SDL initiazlization failed");

    let video_subsystem = sdl_context
        .video()
        .expect("Couldn't get the video subsystem");

    sdl2::image::init(INIT_PNG | INIT_JPG).expect("Couldn't initialize image context");

    let window = video_subsystem
        .window("rust-sdl2 image demo", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .expect("Failed to create window");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Failed to convert window into canvas");

    let texture_creator: TextureCreator<_> = canvas.texture_creator();

    let image_texture = texture_creator
        .load_texture("assets/crying_vscode.jpg")
        .expect("Couldn't load image");

    let mut event_pump = sdl_context
        .event_pump()
        .expect("Failed to get SDL event pump");

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas
            .copy(&image_texture, None, None)
            .expect("Render failed");
        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
