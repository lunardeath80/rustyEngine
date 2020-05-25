pub mod screen;


#[cfg(test)]
mod tests {
    use crate::screen::colour_White;
    use crate::screen::colour_Green;
    use crate::screen::colour_Cyan;

    #[test]
    fn draw_blank_screen() {
        let buff = crate::screen::new(32,32);

        buff.draw();
    }

    #[test]
    fn change_one_pixel() {
        let mut buff = crate::screen::new(32,32);
        buff.set_pixel(3,colour_Green);

        buff.draw();
        std::thread::sleep(std::time::Duration::from_millis(2000));
        buff.set_pixel(5, colour_Cyan);
        buff.draw();
    }

}