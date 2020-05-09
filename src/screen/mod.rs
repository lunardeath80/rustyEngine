pub struct Screen {
    pixels: Vec<char>,
    scale: usize,
    width: usize,
    height: usize
}
pub fn new(width:usize, height: usize)  -> Screen {
    let pixels = vec!['\u{2580}'; width*height];
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
                buffer.push(self.pixels[i+self.width*j]);
            }
            buffer.push_str("#\n");
        }
        buffer.push_str(&"#".repeat(self.scale * self.width + 2));
        buffer.push_str("\n");

        print!("{}",buffer)
    }


}

