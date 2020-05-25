use std::{thread,time};
use std::time::Duration;

mod screen;
mod anim;

use rustyMath::Vector::{*};

fn main() {

    let mut Window = screen::new(64,32);
    print!("\u{001b}[1J");
    print!("\u{001b}[H");
    let start = Vec2::new(0.0, 0.0);
    let end = Vec2::new(16.0, 16.0);

    anim::draw_line(&mut Window, start,end);

}