use crate::grocy::*;

pub enum AppState {
	Loading,
	Stock,
}

pub struct Controller {
	pub model: Grocy,
	pub state: AppState,
	pub system_info: Option<SystemInfo>,
	pub stock: Option<Stock>,
	pub db_changed_time: Option<DbChangedTime>,
	pub index: usize,
}


impl Controller {

	pub fn new(model: Grocy) -> Self{
		Controller {
			model: model,
			state: AppState::Loading,
			system_info: None,
			stock: None,
			db_changed_time: None,
			index: 0,
		}
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

	pub fn on_down(&mut self){
		self.index += 1;
		// fixme: add check for max;
	}

	pub fn on_up(&mut self){
		if self.index > 0{
			self.index -= 1;
		}
	}

	fn check_db_change(&mut self){
		match &self.db_changed_time{
			None => {
				self.db_changed_time = Some(self.model.db_changed_time());
				self.reload_stock();
			},
			Some(_) => {},
		}
	}

	fn reload_stock(&mut self){
		self.stock = Some(self.model.stock());
		match &self.state {
			Loading => { self.state = AppState::Stock; }
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

