use sdl2::pixels::Color;
use sdl2::rect::Rect;

#[derive(Debug, PartialEq)]
pub struct Pixel {
    rect: Rect,
    color: Color,
}

impl Pixel {
    pub fn new(x: u32, y: u32, size: u32) -> Self {
        Self {
            rect: Rect::new(x as i32, y as i32, size, size),
            color: Color::RGB(0, 0, 0),
        }
    }

    pub fn set_color_rgb(&mut self, color: (u8, u8, u8)) {
        self.color.r = color.0;
        self.color.g = color.1;
        self.color.b = color.2;
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn rect(&self)-> &Rect {
        &self.rect
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_pix() {
        let pix = Pixel {
            rect: Rect::new(10, 35, 5, 5),
            color: Color::RGB(0, 0, 0),
        };
        assert_eq!(pix, Pixel::new(10, 35, 5));
    }

    #[test]
    fn test_set_color() {
        let mut pix = Pixel {
            rect: Rect::new(10, 35, 5, 5),
            color: Color::RGB(0, 0, 0),
        };
        pix.set_color_rgb((13, 14, 15));
        assert_eq!(*pix.color(), Color::RGB(13, 14, 15));
    }
}
