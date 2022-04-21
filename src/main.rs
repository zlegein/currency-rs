#[macro_use] extern crate rocket;
use rocket::serde::json::{ json, Value, Json };
use rocket::serde::{ Deserialize, Serialize };
use rusty_money::{ExchangeRate, iso, Money};
use rust_decimal_macros::*;
use rust_decimal::Decimal;
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
    rates: HashMap<String, Decimal>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Rate {
    code: String,
    factor: Decimal,
}

#[derive(Deserialize)]
struct Input<'r> {
    code: &'r str,
    amount: Decimal
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConvertResponse {
    code: String,
    amount: String
}

#[get("/rate/<code>")]
fn rate(code: String) -> Json<Rate> {
    let result = std::fs::read_to_string("./static/currency.json").expect("unable to read file");
    let data: Data = rocket::serde::json::from_str(&result).unwrap(); 
    let rate_option = data.rates.get(&code);
    if let Some(factor) = rate_option { 
        return Json(Rate{ code: code, factor: *factor })
    }
    let value: Decimal = dec!(1);
    Json(Rate{code: "USD".to_string(), factor: value  })
}

#[post("/convert", data="<input>")]
fn convert(input: Json<Input<'_>>) -> Json<ConvertResponse> {
    let result = std::fs::read_to_string("./static/currency.json").expect("unable to read file");
    let data: Data = rocket::serde::json::from_str(&result).unwrap(); 
    let rate_option = data.rates.get(input.code).expect("could not find rate");
    let rate_code = iso::find(input.code).expect("could not find currency code");
    let rate = ExchangeRate::new(iso::USD, rate_code, *rate_option).unwrap();
    let amount = Money::from_decimal(input.amount, iso::USD);
    let money = rate.convert(amount); 
    match money {
        Ok(result) => return Json(ConvertResponse{code: rate_code.to_string(), amount: result.to_string()  }),
        Err(err) => println!("Could not convert {}", err)
    }
    Json(ConvertResponse{code: "USD".to_string(), amount: "1.0".to_string()  })
    
}