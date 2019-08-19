use crate::pixel::Pixel;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;

pub struct Display {
    canvas: WindowCanvas,
    event_pump: EventPump,
    pixels: Vec<Pixel>,
    width: u32,
    height: u32,
}

pub struct DisplayBuilder {
    name: String,
    width: u32,
    height: u32,
    pixel_size: u32,
    margin_x: u32,
    margin_y: u32,
}

impl Display {
    pub fn refresh(&mut self) {
        for pixel in &self.pixels {
            self.canvas.set_draw_color(pixel.color().clone());
            self.canvas.fill_rect(pixel.rect().clone()).unwrap();
        }
        self.canvas.present();
    }

    pub fn get_event_pump(&mut self) -> &mut EventPump {
        &mut self.event_pump
    }

    pub fn pixel_at(&mut self, x: u32, y: u32) -> &mut Pixel {
        &mut self.pixels[(y * self.width + x % self.width) as usize]
    }

    pub fn from_buffer(&mut self, buffer: &[(u8, u8, u8)]) {
        for i in 0..self.pixels.len() {
            self.pixels[i].set_color_rgb(buffer[i]);
        }
    }

    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn width(&self) -> u32 {
        self.width
    }
}

impl DisplayBuilder {
    pub fn new(name: &str, width: u32, height: u32, pixel_size: u32) -> Self {
        Self {
            name: String::from(name),
            width: width,
            height: height,
            margin_x: 0,
            margin_y: 0,
            pixel_size: pixel_size,
        }
    }

    pub fn with_margin(&mut self, margin_x: u32, margin_y: u32) -> &mut Self {
        self.margin_x = margin_x;
        self.margin_y = margin_y;
        self
    }

    pub fn build(&self) -> Result<Display, Box<dyn std::error::Error>> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let win_width = self.width * self.pixel_size + 2 * self.margin_x;
        let win_height = self.height * self.pixel_size + 2 * self.margin_y;
        let window = video_subsystem
            .window(&self.name, win_width, win_height)
            .position_centered()
            .build()?;

        let mut canvas = window.into_canvas().build()?;
        let event_pump = sdl_context.event_pump()?;
        canvas.clear();
        Ok(Display {
            canvas: canvas,
            event_pump: event_pump,
            pixels: self.make_pixel_grid(),
            width: self.width,
            height: self.height,
        })
    }

    fn make_pixel_grid(&self) -> Vec<Pixel> {
        let mut pixels = Vec::with_capacity((self.height * self.width) as usize);
        for j in 0..self.height {
            for i in 0..self.width {
                pixels.push(Pixel::new(
                    i * self.pixel_size + self.margin_x,
                    j * self.pixel_size + self.margin_y,
                    self.pixel_size,
                ));
            }
        }
        pixels
    }
}
