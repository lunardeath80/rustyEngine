mod screen;

fn main() {
    let mut window = screen::new(32, 32, 2).toggle_border();

    window.clear();

    for i in 0..32 * 32 {
        if i % 5 == 0 {
            window.set_pixel(i, screen::COLOUR_RED);
        } else if i % 7 == 3 {
            window.set_pixel(i, screen::COLOUR_GREEN);
        }
    }
    window.draw();
}
