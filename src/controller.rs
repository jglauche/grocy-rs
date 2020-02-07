use crate::grocy::*;

pub enum AppState {
	Startup,
	Main,
}

pub struct Controller {
	pub model: Grocy,
	pub state: AppState,
	pub version: Option<String>,
	pub stock: Option<Vec<StockElement>>,
}


impl Controller {

	pub fn new(model: Grocy) -> Self{
		Controller {
			model: model,
			state: AppState::Main,
			version: None,
			stock: None,
		}
	}


}

