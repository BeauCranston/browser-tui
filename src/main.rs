use std::io;
mod term_ui;
fn main() -> io::Result<()> {
    term_ui::init()
}
