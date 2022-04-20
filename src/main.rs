#[macro_use] extern crate rocket;
use rocket::serde::json::{ json, Value, Json };
use rocket::serde::{ Deserialize, Serialize };
use std::collections::HashMap;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, rates, rate])
}

#[get("/rates")]
fn rates() -> Value {
    let result = std::fs::read_to_string("./static/currency.json").expect("unable to read file");
    json!(result)
}  

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub disclaimer: String,
    pub license: String,
    pub timestamp: i64,
    pub base: String,
    pub rates: HashMap<String, f32>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rate {
    pub code: String,
    pub factor: f32,
}

#[get("/rate/<code>")]
fn rate(code: String) -> Json<Rate> {
    let result = std::fs::read_to_string("./static/currency.json").expect("unable to read file");
    let data: Data = rocket::serde::json::from_str(&result).unwrap(); 
    let rate_option = data.rates.get(&code);
    if let Some(factor) = rate_option { 
        return Json(Rate{ code: code, factor: *factor })
    }
    let value: f32 = 1.0;
    Json(Rate{code: "USD".to_string(), factor: value  })
}