use std::io;

use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    layout::{Constraint, Layout},
    style::Stylize,
    widgets::{Block, Paragraph},
    DefaultTerminal, Frame,
};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = run(terminal);
    ratatui::restore();
    app_result
}

fn run(mut terminal: DefaultTerminal) -> io::Result<()> {
    loop {
        terminal.draw(draw)?;

        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(());
            }
        }
    }
}

fn draw(frame: &mut Frame) {
    let [left, right] = Layout::horizontal([Constraint::Fill(1); 2]).areas(frame.area());
    let [top_right, bottom_right] = Layout::vertical([Constraint::Fill(1); 2]).areas(right);

    frame.render_widget(Block::bordered().title("Left"), left);
    frame.render_widget(Block::bordered().title("Top"), top_right);
    frame.render_widget(Block::bordered().title("Bottom Right"), bottom_right);
}
