use std::io;
use std::io::Write;
use crate::grocy;

pub fn query_config() -> grocy::Grocy{
  print!("Please enter grocy api address: ");
  io::stdout().flush().unwrap();
  let mut uri = String::new();
  io::stdin().read_line(&mut uri).expect("Failed to read line");
	uri = uri.trim().to_string();

  print!("Please enter grocy api key: ");
  io::stdout().flush().unwrap();
  let mut api_key = String::new();
  io::stdin().read_line(&mut api_key).expect("Failed to read line");
	api_key = api_key.trim().to_string();

	grocy::Grocy::from_creds(uri, api_key).expect("failed to get grocy")
}

