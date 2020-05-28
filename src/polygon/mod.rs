extern crate rustyMath;
use rustyMath::Vector::Vec2;
use crate::screen::Screen;
use crate::screen::Blit;

pub struct Polygon {
    vertex: Vec<Vec2>
}

pub fn line(start: Vec2, end: Vec2) -> Polygon {

    Polygon {
        vertex: vec!(start, end)
    }
}

pub fn triangle(a: Vec2, b: Vec2, c: Vec2) -> Polygon{

    Polygon {
        vertex: vec!(a,b,c)
    }
}

impl Blit for Polygon {

    fn blit(&self, Window: &mut Screen) {

        for i in 0..(self.vertex.len()-1) {
            Window.blit_line(self.vertex[i], self.vertex[i+1]);
        }

    }

}

