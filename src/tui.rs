use crate::view;
use crate::controller::Controller;
use std::io::{Write, stdout};
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

use crossterm::{
	event::{self, Event as CEvent, KeyCode},
	execute,
	terminal::{
		disable_raw_mode,
		enable_raw_mode,
		EnterAlternateScreen,
		LeaveAlternateScreen,
	},
};

use tui::{backend::CrosstermBackend, Terminal};
// use tui::widgets::{Widget, Block, Borders};
// use tui::layout::{Layout, Constraint, Direction};

enum Event<I> {
	Input(I),
	Tick,
}

pub fn main(mut ctrl: Controller) -> Result<(), failure::Error> {
	let mut stdout = stdout();
	enable_raw_mode()?;
	execute!(stdout, EnterAlternateScreen)?;
	let backend = CrosstermBackend::new(stdout);
	let mut terminal = Terminal::new(backend)?;
	terminal.clear()?;

	let (tx, rx) = mpsc::channel();

	thread::spawn(move || {
		loop {
			if event::poll(Duration::from_millis(50)).unwrap() {
					if let CEvent::Key(key) = event::read().unwrap() {
							tx.send(Event::Input(key)).unwrap();
					}
			}

			tx.send(Event::Tick).unwrap();
		}
	});

	loop{
		view::draw(&mut terminal, &ctrl)?;
		match rx.recv()? {
			Event::Input(event) => match event.code {
				KeyCode::Char('q') => {
					disable_raw_mode()?;
					execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
					terminal.show_cursor()?;
					break;
				},
				KeyCode::Char(c) => ctrl.on_key(c),
				KeyCode::Up => ctrl.on_up(),
				KeyCode::Down => ctrl.on_down(),
				_ => {},
			},
			Event::Tick => {
				ctrl.on_tick();
			},

		}
	}
	Ok(())
}
