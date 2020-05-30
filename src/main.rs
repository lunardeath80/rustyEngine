use std::{thread,time};
use std::time::Duration;

mod screen;
mod anim;
mod polygon;
mod transform;

use rustyMath::Vector::{*};
use crate::polygon::Polygon;
use crate::screen::Blit;
use crate::transform::Transform;

fn main() {

    let mut window = screen::new(256,48);
    print!("\u{001b}[1J");
    print!("\u{001b}[H");

    let a = Vec2::new(5.0, 0.0);
    let b = Vec2::new(0.0,5.0);
    let c  = Vec2::new(10.0, 5.0);
    let d = Vec2::new(16.0, 2.0);
    let trans = Vec2::new(3.0, 2.0);
    let mut poly = polygon::polygon(vec![a,b,c,d]);

    for i in 0..10 {
        window.clear();
        poly.blit(&mut window);
        window.draw();
        thread::sleep(Duration::from_millis(100));

        poly.translate(trans);
    }
}