use rustyMath::Vector::Vec2;

pub struct Screen {
    pixels: Vec<Pixel>,
    scale: usize,
    width: usize,
    height: usize,
}
#[derive(Clone)]

struct Pixel {
    px: char,
    colour: String,
}

pub const BASE_PIXEL_CHAR: char = '\u{2588}';

pub const colour_Red: &str = "\u{001b}[31m";
pub const colour_Blue: &str = "\u{001b}[34m";
pub const colour_Green: &str = "\u{001b}[32m";
pub const colour_Yellow: &str = "\u{001b}[33m";
pub const colour_Cyan: &str = "\u{001b}[36m";
pub const colour_White: &str = "\u{001b}[37m";

impl Pixel {
    pub fn unwrap(&self) -> String {
        let mut out = String::from("");

        out.push_str(&self.colour);
        out.push(self.px);
        out
    }
}

fn new_pixel(colour: &str) -> Pixel {
    Pixel {
        px: BASE_PIXEL_CHAR,
        colour: String::from(colour),
    }
}
pub fn new(width: usize, height: usize) -> Screen {
    let px = Pixel {
        px: BASE_PIXEL_CHAR,
        colour: String::from(colour_White),
    };

    let pixels = vec![px; width * height];
    Screen {
        pixels,
        scale: 1,
        width,
        height,
    }
}

impl Screen {
    pub fn draw(&self) {
        let mut buffer = String::from("");

        buffer.push_str("\u{001b}[50A");
        buffer.push_str(&"#".repeat(self.scale * self.width + 2));
        buffer.push_str("\n");

        for j in 0..self.height {
            buffer.push_str("#");
            for i in 0..self.width {
                buffer.push_str(&self.pixels[i + self.width * j].unwrap());
            }
            buffer.push_str("#\n");
        }
        buffer.push_str(&"#".repeat(self.scale * self.width + 2));
        buffer.push_str("\n");

        print!("{}", buffer)
    }

    pub fn set_pixel(&mut self, position: usize, colour: &str) {
        self.pixels[position] = new_pixel(colour);
    }

    pub fn flatten(&self, vec_pos: Vec2) -> usize {
        (vec_pos.x as usize) + self.width * usize::from(vec_pos.y as usize)
    }

    pub fn blit_line(&mut self, start: Vec2, end: Vec2) {
        let dir = (end.grid() - start.grid()).normalise();
        let mut cur = start.grid();

        while cur.grid() != end {
            self.set_pixel(self.flatten(cur.grid()), colour_Green);
            cur = (dir + cur);
        }
    }

    pub fn clear(&mut self) {
        for i in 0..self.width * self.height {
            self.set_pixel(i, colour_White);
        }
        //let px: Vec<usize> = (0..self.width*self.height).collect();
        //px.iter().map(|x| self.set_pixel(*x, colour_Red));
    }
}

//Trait that handles rendering of types
pub trait Blit {
    fn blit(&self, Window: &mut Screen);
}
