extern crate dirs;
extern crate restson;
extern crate serde_aux;

use serde::{Serialize, Deserialize, Deserializer};
use std::fs;
use std::io;
use std::io::Write;
use std::fs::File;
use std::str::FromStr;
use std::fmt::Display;
use restson::{RestClient,RestPath,Error};
use chrono::{DateTime, Utc};
use serde_aux::prelude::deserialize_bool_from_anything;

pub fn deserialize_number<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
	D: Deserializer<'de>,
	T: FromStr + serde::Deserialize<'de>,
	<T as FromStr>::Err: Display,
{
	#[derive(Deserialize)]
	#[serde(untagged)]
	enum StringOrInt<T> {
		String(String),
		Number(T),
	}

	match StringOrInt::<T>::deserialize(deserializer)? {
		StringOrInt::String(s) => if s.is_empty() { "0" } else { &s }.parse::<T>().map_err(serde::de::Error::custom),
		StringOrInt::Number(i) => Ok(i),
	}
}


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

#[derive(Serialize,Deserialize,Debug)]
#[serde(untagged)]
enum Stock {
	Array(Vec<StockElement>)
}

#[derive(Serialize,Deserialize,Debug)]
pub struct StockElement {
	#[serde(deserialize_with="deserialize_number")]
	pub product_id: u32,
	#[serde(deserialize_with="deserialize_number")]
	pub amount: u32,
	#[serde(deserialize_with="deserialize_number")]
	pub amount_aggregated: u32,
	#[serde(deserialize_with="deserialize_number")]
	pub amount_opened: u32,
	#[serde(deserialize_with="deserialize_number")]
	pub amount_opened_aggregated: u32,
	//fixme Date without time
	pub best_before_date: String,
	#[serde(deserialize_with = "deserialize_bool_from_anything")]
	pub is_aggregated_amount: bool,
	pub product: Product,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct OptionalU32( #[serde(deserialize_with="deserialize_number")] u32 );

#[derive(Serialize,Deserialize,Debug)]
#[serde(untagged)]
enum Products {
	Array(Vec<Product>)
}

#[derive(Serialize,Deserialize,Debug)]
pub struct Product {
	#[serde(deserialize_with="deserialize_number")]
	pub id: u32,
	pub name: String,
	pub description: Option<String>,
	pub location_id: Option<OptionalU32>,
	#[serde(deserialize_with="deserialize_number")]
	pub qu_id_purchase: u32,
	#[serde(deserialize_with="deserialize_number")]
	pub qu_id_stock: u32,
	// FIXME decimal #[serde(deserialize_with="deserialize_number")]
	pub qu_factor_purchase_to_stock: String,
	pub barcode: String,
	#[serde(deserialize_with="deserialize_number")]
	pub min_stock_amount: u32,
	#[serde(deserialize_with="deserialize_number")]
	pub default_best_before_days: i32,
	#[serde(with = "grocy_datetime_format")]
	pub row_created_timestamp: DateTime<Utc>,

	pub product_group_id: Option<OptionalU32>,
	pub picture_file_name: Option<String>,

	#[serde(deserialize_with="deserialize_number")]
	pub default_best_before_days_after_open: u32,

	#[serde(deserialize_with = "deserialize_bool_from_anything")]
	allow_partial_units_in_stock: bool,
	#[serde(deserialize_with = "deserialize_bool_from_anything")]
	enable_tare_weight_handling: bool,
	// Fixme
	pub tare_weight: String,
	#[serde(deserialize_with = "deserialize_bool_from_anything")]
	pub not_check_stock_fulfillment_for_recipes: bool,

	pub parent_product_id: Option<OptionalU32>,

	// Fixme
	pub calories: Option<String>,
	#[serde(deserialize_with="deserialize_number")]
	pub cumulate_min_stock_amount_of_sub_products: u32,
	#[serde(deserialize_with="deserialize_number")]
	pub default_best_before_days_after_freezing: u32,
	#[serde(deserialize_with="deserialize_number")]
	pub default_best_before_days_after_thawing: u32,

}

#[derive(Serialize,Deserialize,Debug)]
#[serde(untagged)]
enum Locations {
	Array(Vec<Location>)
}

#[derive(Serialize,Deserialize,Debug)]
pub struct Location {
	#[serde(deserialize_with="deserialize_number")]
	pub id: u32,
	pub name: String,
	pub description: Option<String>,
	#[serde(with = "grocy_datetime_format")]
	pub row_created_timestamp: DateTime<Utc>,
	#[serde(deserialize_with = "deserialize_bool_from_anything")]
	pub is_freezer: bool,
}

impl RestPath<()> for SystemInfo { fn get_path(_: ()) -> Result<String,Error> { Ok(String::from("/api/system/info"))}}
impl RestPath<()> for DbChangedTime { fn get_path(_: ()) -> Result<String,Error> { Ok(String::from("/api/system/db-changed-time"))}}
impl RestPath<()> for Stock { fn get_path(_: ()) -> Result<String,Error> { Ok(String::from("/api/stock"))}}
impl RestPath<()> for Locations { fn get_path(_: ()) -> Result<String,Error> { Ok(String::from("/api/objects/locations"))}}
impl RestPath<()> for Products { fn get_path(_: ()) -> Result<String,Error> { Ok(String::from("/api/objects/products"))}}

impl RestPath<u32> for Product { fn get_path(param: u32) -> Result<String,Error> { Ok(format!("/api/objects/products/{}", param))}}




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


	pub fn system_info(&self) -> SystemInfo { self.client().get(()).unwrap() }

	pub fn db_changed_time(&self) {
		let data: DbChangedTime = self.client().get(()).unwrap();
		println!("{:?}", data);
	}

	pub fn stock(&self) {
		let data: Stock = self.client().get(()).unwrap();
		println!("#{:?}", data);
	}

	pub fn locations(&self) {
		let data: Locations = self.client().get(()).unwrap();
		println!("#{:?}", data);
	}

	pub fn products(&self) {
		let data: Products = self.client().get(()).unwrap();
		match data {
			Products::Array(a) => {
				for x in a.iter() {
					println!("{} {}", x.id, x.name);
				}
			}
		}
	}

	pub fn product(&self, param: u32) {
		let data: Product = self.client().get(param).unwrap();
		println!("{:?}", data);
	}


// for testing json files
// let conf = fs::read_to_string("test.json").expect("");
// let data: MyData = serde_json::from_str(&conf).expect("");

}
