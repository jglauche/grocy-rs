pub mod view;
pub mod grocy;
pub mod controller;

pub mod cli;
pub mod tui;
//use std::thread;

use controller::Controller;

fn main() {
	let grocy = match grocy::Grocy::from_config("api.json"){
		Ok(res) => {
			res
		}
		Err(_) => {
			cli::query_config()
		}
	};
	let controller = Controller::new(grocy);

	tui::main(controller).expect("");
//	grocy.system_info();
//	grocy.db_changed_time();
//	grocy.locations();
//	grocy.stock();
	// grocy.products();

//	grocy.product(11);


}
