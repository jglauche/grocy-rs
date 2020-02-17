use crate::grocy::*;
extern crate enum_map;
use enum_map::{Enum, EnumMap, enum_map};

#[derive(Copy, Clone, Enum, PartialEq, Eq, Hash)]
pub enum AppState {
	Loading,
	Stock,
	Locations,
}

pub struct Controller {
	pub model: Grocy,
	pub state: AppState,
	pub system_info: Option<SystemInfo>,
	pub stock: Option<Vec<StockElement>>,
	pub locations: Option<Vec<Location>>,
	pub db_changed_time: Option<DbChangedTime>,
	pub index: EnumMap<AppState, usize>,
}


impl Controller {
	pub fn new(model: Grocy) -> Self{
		Controller {
			model: model,
			state: AppState::Loading,
			system_info: None,
			stock: None,
			locations: None,
			db_changed_time: None,
			index: enum_map!{
				AppState::Loading => 0,
				AppState::Stock => 0,
				AppState::Locations => 0,
			},
		}
	}

	pub fn data(&self) -> Vec<String> {
		match &self.state {
			AppState::Stock => self.stock.as_ref().map(|a| a.iter().map(ToString::to_string).collect::<Vec<_>>() ),
			AppState::Locations => self.locations.as_ref().map(|a| a.iter().map(ToString::to_string).collect::<Vec<_>>() ),
			_ => None,
		}.unwrap_or_else(Vec::new)
	}

	pub fn on_tick(&mut self) {
		match &self.system_info{
			None => self.get_system_info(),
			Some(_) => {},
		}
		self.check_db_change();
	}

	pub fn on_key(&mut self, key: char) {
		match key {
			'r' => {},
			_ => {},
		}
	}

	pub fn on_down(&mut self) {
		let len = self.len();
		if len == 0 {
			return
		}

		if self.index[self.state] < len - 1 {
			 self.index[self.state] += 1;
		}
	}

	pub fn on_up(&mut self) {
		if self.index[self.state] > 0{
			self.index[self.state] -= 1;
		}
	}

	pub fn on_pageup(&mut self) {
		if self.index[self.state] < 15 {
			self.index[self.state] = 0;
		} else {
			self.index[self.state] -= 15;
		}
	}

	pub fn on_pagedown(&mut self) {
		if self.len() == 0{
			return
		}

		if self.index[self.state] + 15 < self.len() - 1 {
			self.index[self.state] += 15;
		} else {
			self.index[self.state] = self.len() - 1;
		}
	}

	// returns the length of the current list
	pub fn len(&mut self) -> usize {
		match &self.state {
			AppState::Stock => {
				match &self.stock{
					Some(a) => a.len(),
					None => 0,
				}
			},
			_ => 0,
		}
	}

	fn check_db_change(&mut self){
		match &self.db_changed_time{
			None => {
				self.db_changed_time = Some(self.model.db_changed_time());
				// self.reload_locations();
				self.reload_stock();
			},
			Some(_) => {},
		}
	}

	fn reload_locations(&mut self){
		self.locations = Some(self.model.locations());
	}

	fn reload_stock(&mut self){
		self.stock = Some(self.model.stock());
		match &self.state {
			AppState::Loading => { self.state = AppState::Stock; }
			_ => {},
		}
	}

	fn get_system_info(&mut self) {
		self.system_info = Some(self.model.system_info());
	}

	pub fn print_system_info(&self) -> String {
		match &self.system_info{
			None => String::from("Not connected"),
			Some(s) => format!("Grocy version {}", &s.grocy_version.version)
		}
	}


}

