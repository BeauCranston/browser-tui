use std::io;

use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    layout::{Alignment, Constraint, Direction, Layout},
    style::Stylize,
    widgets::{Block, Borders, Paragraph},
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
    let area = frame.area();
    let layout = Layout::vertical([Constraint::Length(3), Constraint::Ratio(8, 10)]);
    let [search, results] = layout.areas(area);

    frame.render_widget(
        Block::default()
            .title("Search")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL),
        search,
    );
    frame.render_widget(Block::default().borders(Borders::ALL), results);
}
