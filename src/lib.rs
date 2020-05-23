extern crate rustyMath;
pub mod screen;


#[cfg(test)]
mod tests {
    #[test]
    fn draw_blank_screen() {
        let buff = crate::screen::new(32,32);

        buff.draw();
    }

}