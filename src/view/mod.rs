pub use crate::controller::{AppState, Controller};
pub use crate::grocy::*;

use tui::backend::Backend;
use tui::{Frame, Terminal};
use std::io;

use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Text, SelectableList, Paragraph, Widget};
use tui::layout::{Layout, Constraint, Rect};

fn title_style() -> tui::style::Style {
	Style::default().fg(Color::Magenta).modifier(Modifier::BOLD)
}

fn mb<'a>(title: &'a str) -> tui::widgets::Block<'a> {
	Block::default().title(title).borders(Borders::ALL).title_style(title_style())
}



pub fn draw<B: Backend>(terminal: &mut Terminal<B>, ctrl: &Controller) -> Result<(), io::Error> {
	terminal.draw(|mut f| {
		let chunks = Layout::default()
		.constraints([Constraint::Percentage(100)].as_ref())
		.split(f.size());

		match &ctrl.state {
			_ => draw_main(&mut f, &ctrl, chunks[0]),
			// _ => {}
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
				Constraint::Percentage(90),
			]
			.as_ref(),
		)
		.split(area);
	draw_contents(f, ctrl, chunks[0]);
}

fn draw_contents<B>(f: &mut Frame<B>, ctrl: &Controller, area: Rect)
where
	B: Backend,
{

	match &ctrl.state {
		AppState::Loading => {
			let text = [
				Text::raw("Loading data..."),
			];
			Paragraph::new(text.iter())
			.block(mb(&ctrl.print_system_info()))
			.render(f, area);
		},
		AppState::Stock | AppState::Locations => {
			// TODO: try to move this logic into controller.
			//
			// Controller does match state, returns Vector of items as unified type, can chain more types to one type of view
			SelectableList::default()
			.block(mb(&ctrl.print_system_info()))
			.items(&ctrl.data())
			.select(Some(ctrl.index[ctrl.state]))
			.style(Style::default().fg(Color::White))
			.highlight_style(Style::default().fg(Color::Magenta).modifier(Modifier::BOLD))
			.highlight_symbol(">>")
			.render(f, area);
		}
	}
}

