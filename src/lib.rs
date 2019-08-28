pub mod display;
pub mod pixel;

pub use sdl2;

#[cfg(test)]
mod tests {
    extern crate rand;

    use super::*;
    use display::DisplayBuilder;
    use rand::prelude::*;
    use sdl2::audio::{AudioCallback, AudioSpecDesired};
    use sdl2::event::Event;
    use sdl2::keyboard::Keycode;
    struct SquareWave {
        phase_inc: f32,
        phase: f32,
        volume: f32,
    }

    impl AudioCallback for SquareWave {
        type Channel = f32;

        fn callback(&mut self, out: &mut [f32]) {
            for x in out.iter_mut() {
                *x = if self.phase <= 0.5 {
                    self.volume
                } else {
                    -self.volume
                };
                self.phase = (self.phase + self.phase_inc) % 1.0;
            }
        }
    }

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

    #[test]
    fn test_sound() {
        let mut display = DisplayBuilder::new("Display", 64, 32, 10)
            .with_margin(5, 5)
            .build()
            .unwrap();
        let audio_subsystem = display.context.audio().unwrap();

        let desired_spec = AudioSpecDesired {
            freq: Some(44100),
            channels: Some(1),
            samples: None,
        };
        let device = audio_subsystem
            .open_playback(None, &desired_spec, |spec| SquareWave {
                phase_inc: 440.0 / spec.freq as f32,
                phase: 0.0,
                volume: 0.25,
            })
            .unwrap();
        device.resume();
        std::thread::sleep(std::time::Duration::from_millis(2000));
        device.pause();
        std::thread::sleep(std::time::Duration::from_millis(2000));
        device.resume();
        std::thread::sleep(std::time::Duration::from_millis(2000));
    }
}
