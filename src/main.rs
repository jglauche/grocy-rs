pub mod grocy;
pub mod cli;


fn main() {
	let grocy = match grocy::Grocy::from_config("api.json"){
		Ok(res) => {
			res
		}
		Err(_) => {
			cli::query_config()
		}
	};
//	grocy.system_info();
//	grocy.db_changed_time();
//	grocy.locations();
//	grocy.stock();
	grocy.product();



}
