use crate::view;
use crate::controller::Controller;
use std::io::{Write, stdout};
use crossterm::{
//	event::{self, Event as CEvent, KeyCode},
	execute,
	terminal::{
//		disable_raw_mode,
//		enable_raw_mode,
		EnterAlternateScreen
	},
};

use tui::{backend::CrosstermBackend, Terminal};
// use tui::widgets::{Widget, Block, Borders};
// use tui::layout::{Layout, Constraint, Direction};

pub fn main(ctrl: Controller) -> Result<(), failure::Error> {
	let mut stdout = stdout();

	execute!(stdout, EnterAlternateScreen)?;
	let backend = CrosstermBackend::new(stdout);
	let mut terminal = Terminal::new(backend)?;
  terminal.clear()?;

	loop{
		view::draw(&mut terminal, &ctrl)?;
	}
	Ok(())
}
