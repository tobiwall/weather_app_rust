use reqwest::blocking::get;
use serde_json::{from_str, Value};
use std::io::{self, Write};

mod env;

fn main() {
    loop {
        print!("Bitte geben Sie eine Stadt ein: ");
        io::stdout().flush().unwrap();
        let mut city = String::new();
        io::stdin()
            .read_line(&mut city)
            .expect("Fehler beim Lesen der Eingabe");

        city = city.trim().to_string();

        let api_key = env::API_KEY;
        let url = format!(
            "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric&lang=de",
            city, api_key
        );

        let response = match get(url) {
            Ok(resp) => resp,
            Err(e) => {
                println!("Fehler beim Abrufen der URL: {}", e);
                continue;
            }
        };

        let response_as_text = response.text().unwrap();
        let json: Value = match from_str(&response_as_text) {
            Ok(val) => val,
            Err(e) => {
                println!("Fehler beim Verarbeiten des JSONs: {}", e);
                continue;
            }
        };

        let temp = match json["main"]["temp"].as_f64() {
            Some(t) => t,
            None => {
                println!("Konnte die Temperatur nicht abrufen.");
                continue;
            }
        };
        let rounded_temp = temp.round();

        let description = match json["weather"][0]["description"].as_str() {
            Some(desc) => desc,
            None => {
                println!("Konnte die Wetterbeschreibung nicht abrufen.");
                continue;
            }
        };
        let description_lower_case = to_lowercase_first_char(description);

        let humidity = match json["main"]["humidity"].as_u64() {
            Some(h) => h,
            None => {
                println!("Konnte die Luftfeuchtigkeit nicht abrufen.");
                continue;
            }
        };

        println!("");
        println!(
            "In {} ist es gerade {}°C, außerdem {} und die Luftfeuchtigkeit beträgt {}%",
            city, rounded_temp, description_lower_case, humidity
        );
        println!("\n");
    }
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
