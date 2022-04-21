#[macro_use] extern crate rocket;
use rocket::response::status::NotFound;
use rocket::serde::json::{ json, Value, Json };
use rocket::serde::{ Deserialize, Serialize };
use rusty_money::iso::Currency;
use rusty_money::{ExchangeRate, iso, Money, MoneyError};
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
    amount: Decimal,
    formatted: String
}

#[get("/rate/<code>")]
fn rate(code: &str) -> Result<Json<Rate>, NotFound<String>> {
    let rate = get_rate(code.to_string());
    if let Some(result) = rate {
        return Ok(Json(Rate{ code: code.to_string(), factor: result }));
    }
    Err(NotFound("Could not find requested currency code.".to_string()))
}

#[post("/convert", data="<input>")]
fn convert(input: Json<Input<'_>>) -> Result<Json<ConvertResponse>, NotFound<String>> {
    
    let amount = Money::from_decimal(Decimal::from_f32_retain(input.amount).unwrap(), iso::USD);
    let money = exchange(amount, input.code.to_string());
    match money {
        Ok(result) => return Ok(Json(ConvertResponse{code: input.code.to_string(), amount: *result.amount(), formatted: format!("{}", result) })),
        Err(err) => Err(NotFound(format!("Currency not found - :{}", err)))
    }
}

fn exchange(amount: Money<Currency>, code: String) -> Result<Money<Currency>, MoneyError> {
    let rate = get_rate(code.to_string()).expect("could not get rate");
    let rate_code = iso::find(&code[..]).expect("could not find currency");
    let rate = ExchangeRate::new(iso::USD, rate_code, Decimal::from_f32_retain(rate).unwrap()).unwrap();
    rate.convert(amount)
}

fn get_rate(code: String) -> Option<f32> {
    let result = std::fs::read_to_string("./static/currency.json").expect("unable to read file");
    let data: Data = rocket::serde::json::from_str(&result).unwrap(); 
    let rate = data.rates.get(&code);
    match rate {
        Some(result) => return Some(*result),
        None => None
    }
}