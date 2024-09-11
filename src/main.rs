use reqwest::blocking::get;
use serde_json::{from_str, Value};
use std::io::{self, Write};

mod env;

fn main() {
    print!("Bitte geben Sie eine Stadt ein: ");
    io::stdout().flush().unwrap();
    let mut city = String::new();
    io::stdin()
        .read_line(&mut city)
        .expect("Fehler beim Lesen der Eingabe");

    let api_key = env::API_KEY;
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric&lang=de",
        city, api_key
    );

    city = city.trim().to_string();
    let response = get(url).unwrap();
    let response_as_text = response.text().unwrap();
    let json: Value = from_str(&response_as_text).expect("JSON was not well-formatted");
    let temp = &json["main"]["temp"].as_f64().unwrap();
    let rounded_temp = temp.round();
    let description = &json["weather"][0]["description"];
    let description_str = description.as_str().unwrap_or("");
    let description_lower_case = to_lowercase_first_char(description_str);
    let humidity = &json["main"]["humidity"].as_u64().unwrap();

    println!("");
    println!(
        "In {:?} ist es gerade {}°C, außerdem {} und die Luftfeuchtigkeit beträgt {}%",
        city, rounded_temp, description_lower_case, humidity
    );

    println!("");
    println!("");
}

fn to_lowercase_first_char(s: &str) -> String {
    if s.is_empty() {
        return s.to_string();
    }

    let mut chars = s.chars();
    let first_char = chars.next().unwrap();
    let rest_of_string: String = chars.collect();

    let lower_first_char = first_char.to_lowercase();
    let result = format!("{}{}", lower_first_char, rest_of_string);
    
    result
}