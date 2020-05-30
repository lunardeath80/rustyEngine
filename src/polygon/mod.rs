extern crate rustyMath;
use rustyMath::Vector::Vec2;
use crate::screen::Screen;
use crate::screen::Blit;
use crate::transform::Transform;


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

pub fn polygon(vertex: Vec<Vec2>) -> Polygon {

    Polygon {
        vertex
    }

}

impl Blit for Polygon {

    fn blit(&self, Window: &mut Screen) {

        for i in 0..(self.vertex.len()) {
            Window.blit_line(self.vertex[i%self.vertex.len()], self.vertex[(i+1)%self.vertex.len()]);
        }

    }

}

impl Transform for Polygon {

    fn translate(&mut self, vec: Vec2) {
        for v in &mut self.vertex {
            *v = *v+vec;
        }
    }

    fn rotate (&mut self, angle: f64) {

        for v in &mut self.vertex {
            *v = v.rotate(angle.to_radians()).grid();


        }
    }

    fn scale (&mut self, scale: isize) {

    }

}

