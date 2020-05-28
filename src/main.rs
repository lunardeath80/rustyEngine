use std::{thread,time};
use std::time::Duration;

mod screen;
mod anim;
mod polygon;

use rustyMath::Vector::{*};
use crate::polygon::Polygon;
use crate::screen::Blit;

fn main() {

    let mut window = screen::new(256,48);
    print!("\u{001b}[1J");
    print!("\u{001b}[H");

    let a = Vec2::new(0.0, 5.0);
    let b = Vec2::new(5.0,0.0);

    let line = polygon::line(a,b);

    line.blit(&mut window);
    window.draw();
}