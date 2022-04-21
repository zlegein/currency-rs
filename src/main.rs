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
    rocket::build().mount("/", routes![index, rates, rate, convert])
}

#[get("/rates")]
fn rates() -> Value {
    let result = std::fs::read_to_string("./static/currency.json").expect("unable to read file");
    json!(result)
}  

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Data {
    disclaimer: String,
    license: String,
    timestamp: i64,
    base: String,
    rates: HashMap<String, f32>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Rate {
    code: String,
    factor: f32,
}

#[derive(Deserialize)]
struct Input<'r> {
    code: &'r str,
    amount: f32
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConvertResponse {
    code: String,
    amount: f32
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

#[post("/convert", data="<input>")]
fn convert(input: Json<Input<'_>>) -> Json<ConvertResponse> {
    let result = std::fs::read_to_string("./static/currency.json").expect("unable to read file");
    let data: Data = rocket::serde::json::from_str(&result).unwrap(); 
    let rate_option = data.rates.get(input.code);
    if let Some(factor) = rate_option { 
        return Json(ConvertResponse{ code: input.code.to_string(), amount: factor * input.amount })
    }
    Json(ConvertResponse{code: "USD".to_string(), amount: input.amount  })
}