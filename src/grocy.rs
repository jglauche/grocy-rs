extern crate dirs;
extern crate restson;

use serde::{Serialize, Deserialize};
use std::fs;
use std::io;
use std::io::Write;
use std::fs::File;
use restson::{RestClient,RestPath,Error};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize)]
pub struct Grocy{
	pub uri: String,
	api_key: String,
}

fn config_file(name:&str) -> std::path::PathBuf {
	let mut path = dirs::config_dir().unwrap();
	path.push("grocy-rs");

	std::fs::create_dir_all(&path).unwrap_or_else( |_|
		panic!("cannot create path {:?}", path)
	);

	path.push(name);
	path
}

mod grocy_datetime_format{
	use chrono::{DateTime, Utc, TimeZone};
	use serde::{self, Deserialize, Serializer, Deserializer};
	const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

	pub fn serialize<S>(
			date: &DateTime<Utc>,
			serializer: S,
	) -> Result<S::Ok, S::Error>
	where
	S: Serializer,
	{
			let s = format!("{}", date.format(FORMAT));
			serializer.serialize_str(&s)
	}

	pub fn deserialize<'de, D>(
        deserializer: D,
  ) -> Result<DateTime<Utc>, D::Error>
  where
	D: Deserializer<'de>,
	{
			let s = String::deserialize(deserializer)?;
			Utc.datetime_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
	}
}


#[derive(Serialize,Deserialize,Debug)]
#[serde(rename_all = "PascalCase")]
pub struct GrocyVersion {
	pub version: String,
	pub release_date: String,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct SystemInfo {
    pub grocy_version: GrocyVersion,
    pub php_version: String,
    pub sqlite_version: String,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct DbChangedTime {
	#[serde(with = "grocy_datetime_format")]
	pub changed_time: DateTime<Utc>,
}

impl RestPath<()> for SystemInfo { fn get_path(_: ()) -> Result<String,Error> { Ok(String::from("/api/system/info"))}}
impl RestPath<()> for DbChangedTime { fn get_path(_: ()) -> Result<String,Error> { Ok(String::from("/api/system/db-changed-time"))}}


impl Grocy{

	pub fn from_creds(uri: String, api_key: String) -> serde_json::Result<Self>{
		let grocy = Self { uri, api_key };
		let json = serde_json::to_string(&grocy)?;
		let mut f = File::create(config_file("api.json")).expect("failed to create file");
		f.write_all(&json.into_bytes()).expect("failed to write file");

		Ok(grocy)
	}

	pub fn from_config(name:&str) -> io::Result<Self> {
		let f = config_file(&name);
		let conf = fs::read_to_string(f)?;
		let res = serde_json::from_str(&conf)?;
		Ok(res)
	}

	fn client(&self) -> RestClient {
		let mut client = RestClient::new(&self.uri).unwrap();
		client.set_header("GROCY-API-KEY", &self.api_key).unwrap();
		client.set_header("User-Agent", "Grocy Rust Client").unwrap();
		client
	}


	pub fn system_info(&self) {
		let data: SystemInfo = self.client().get(()).unwrap();
		println!("{:?}", data);
	}

	pub fn db_changed_time(&self) {
		let data: DbChangedTime = self.client().get(()).unwrap();
		println!("{:?}", data);
	}


}
