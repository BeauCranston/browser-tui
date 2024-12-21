use std::io;

use wv_window::init_web_view;
mod term_ui;
mod wv_window;
fn main() -> io::Result<()> {
    // term_ui::init()

    init_web_view();

    Ok(())
}
