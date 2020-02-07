use crate::grocy::*;

pub enum AppState {
	Startup,
	Main,
}

pub struct Controller {
	pub model: Grocy,
	pub state: AppState,
	pub system_info: Option<SystemInfo>,
	pub stock: Option<Vec<StockElement>>,
}


impl Controller {

	pub fn new(model: Grocy) -> Self{
		Controller {
			model: model,
			state: AppState::Main,
			system_info: None,
			stock: None,
		}
	}

	pub fn on_tick(&mut self) {
		match &self.system_info{
			None => self.get_system_info(),
			Some(_) => {},
		}
	}

	pub fn on_key(&mut self, key: char) {
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

