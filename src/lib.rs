pub mod display;
pub mod pixel;

pub use sdl2;

#[cfg(test)]
mod tests {
    extern crate rand;

    use super::*;
    use display::DisplayBuilder;
    use rand::prelude::*;
    use sdl2::event::Event;
    use sdl2::keyboard::Keycode;

    #[test]
    fn test_rand_pix() {
        let mut display = DisplayBuilder::new("Display", 64, 32, 10)
            .with_margin(5, 5)
            .build()
            .unwrap();
        let mut rng = rand::thread_rng();
        'main: loop {
            for event in display.get_event_pump().poll_iter() {
                match event {
                    Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        println!("escape received");
                        break 'main;
                    }
                    _ => {}
                }
            }
            let mut buffer = [(0, 0, 0); 64 * 32];
            buffer[0] = (255, 255, 255);
            buffer[1] = (255, 255, 255);
            buffer[64] = (255, 255, 255);
            buffer[65] = (255, 255, 255);
            display.from_buffer(&buffer);
            display.refresh();
            std::thread::sleep(std::time::Duration::from_millis(20));
        }
    }
}
