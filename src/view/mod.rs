pub use crate::controller::Controller;

use tui::backend::Backend;
use tui::{Frame, Terminal};
use std::io;

use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Text, Paragraph, Widget};
use tui::layout::{Layout, Constraint, Rect};



pub fn draw<B: Backend>(terminal: &mut Terminal<B>, ctrl: &Controller) -> Result<(), io::Error> {
 terminal.draw(|mut f| {
		let chunks = Layout::default()
			.constraints([Constraint::Min(10), Constraint::Min(0)].as_ref())
			.split(f.size());

		match &ctrl.state {
			Main => draw_main(&mut f, &ctrl, chunks[0]),
			_ => {}
		};
	})
}

fn draw_main<B>(f: &mut Frame<B>, ctrl: &Controller, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Length(7),
                Constraint::Min(7),
                Constraint::Length(7),
            ]
            .as_ref(),
        )
        .split(area);
    draw_textbox(f, chunks[0]);
}


fn draw_textbox<B>(f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let text = [
        Text::raw("This is a paragraph with several lines. You can change style your text the way you want.\n\nFox example: "),
        Text::styled("under", Style::default().fg(Color::Red)),
        Text::raw(" "),
        Text::styled("the", Style::default().fg(Color::Green)),
        Text::raw(" "),
        Text::styled("rainbow", Style::default().fg(Color::Blue)),
        Text::raw(".\nOh and if you didn't "),
        Text::styled("notice", Style::default().modifier(Modifier::ITALIC)),
        Text::raw(" you can "),
        Text::styled("automatically", Style::default().modifier(Modifier::BOLD)),
        Text::raw(" "),
        Text::styled("wrap", Style::default().modifier(Modifier::REVERSED)),
        Text::raw(" your "),
        Text::styled("text", Style::default().modifier(Modifier::UNDERLINED)),
        Text::raw(".\nOne more thing is that it should display unicode characters: 10€")
    ];
    Paragraph::new(text.iter())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Grocy")
                .title_style(Style::default().fg(Color::Magenta).modifier(Modifier::BOLD)),
        )
        .wrap(true)
        .render(f, area);
}

