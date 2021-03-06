use rustyMath::Vector::Vec2;

pub struct Screen {
    pixels: Vec<Pixel>,
    ratio: usize,
    width: usize,
    height: usize,

    enable_border: bool,
}
#[derive(Clone)]

struct Pixel {
    px: char,
    colour: String,
}

pub const BASE_PIXEL_CHAR: char = '\u{2588}';

pub const COLOUR_RED: &str = "\u{001b}[31m";
pub const COLOUR_BLUE: &str = "\u{001b}[34m";
pub const COLOUR_GREEN: &str = "\u{001b}[32m";
pub const COLOUR_YELLOW: &str = "\u{001b}[33m";
pub const COLOUR_CYAN: &str = "\u{001b}[36m";
pub const COLOUR_WHITE: &str = "\u{001b}[37m";

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
pub fn new(width: usize, height: usize, ratio: usize) -> Screen {
    let px = Pixel {
        px: BASE_PIXEL_CHAR,
        colour: String::from(COLOUR_WHITE),
    };

    let pixels = vec![px; width * height];

    let mut screen = Screen {
        pixels,
        ratio,
        width,
        height,
        enable_border: true,
    };

    screen.clear();

    return screen;
}

impl Screen {
    pub fn draw(&self) {
        let mut buffer = String::from("");

        buffer.push_str("\u{001b}[2J");

        if self.enable_border {
            buffer.push_str(&"#".repeat(self.ratio * self.width + 2));
            buffer.push_str("\n");
        }

        for j in 0..self.height {
            if self.enable_border {
                buffer.push_str("#");
            }
            for i in 0..self.width {
                buffer.push_str(&self.pixels[i + self.width * j].unwrap().repeat(self.ratio));
            }

            if self.enable_border {
                buffer.push_str("\u{001b}[;;m#");
            }

            buffer.push_str("\n");
        }

        if self.enable_border {
            buffer.push_str(&"#".repeat(self.ratio * self.width + 2));
            buffer.push_str("\n");
        }

        print!("{}", buffer)
    }

    pub fn toggle_border(mut self) -> Self {
        self.enable_border = !self.enable_border;

        return self;
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
            self.set_pixel(self.flatten(cur.grid()), COLOUR_GREEN);
            cur = dir + cur;
        }
    }

    pub fn clear(&mut self) {
        for i in 0..self.width * self.height {
            self.set_pixel(i, COLOUR_WHITE);
        }
    }
}

//Trait that handles rendering of types
pub trait Blit {
    fn blit(&self, window: &mut Screen);
}
