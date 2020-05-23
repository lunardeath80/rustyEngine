pub struct Screen {
    pixels: Vec<Pixel>,
    scale: usize,
    width: usize,
    height: usize
}
#[derive(Clone)]

struct Pixel {
    px: char,
    colour: String
}

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

pub fn new(width:usize, height: usize)  -> Screen {
    let px = Pixel {
        px: '\u{2580}',
        colour: String::from(colour_Blue)
    };

    let pixels = vec![px;width*height];
    Screen {
        pixels,
        scale: 1,
        width,
        height
    }
}

impl Screen {

    pub fn draw(&self) {
        let mut buffer = String::from("");

        buffer.push_str(&"#".repeat(self.scale * self.width + 2));
        buffer.push_str("\n");

        for j in 0..self.height {
            buffer.push_str("#");
            for i in 0..self.width {
                buffer.push_str(&self.pixels[i+self.width*j].unwrap());
            }
            buffer.push_str("#\n");
        }
        buffer.push_str(&"#".repeat(self.scale * self.width + 2));
        buffer.push_str("\n");

        print!("{}",buffer)
    }


}

