use std::{thread,time};
use std::time::Duration;

mod screen;


fn main() {

    let mut Window = screen::new(64,32);

    for i in 1..64*32 {

        Window.set_pixel(i-1,screen::colour_White);
        Window.set_pixel(i,screen::colour_Red);
        Window.draw();
        thread::sleep(Duration::from_millis(100));
    }


    print!("\u{001b}[34B");
}