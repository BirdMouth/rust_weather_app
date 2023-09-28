use std::io::stdin;
use serde_json::Value;

static CITY_NAME_PLACEHOLDER: &str = "[CITY_NAME]";

fn main() {
    let mut city_name: String = String::new();

    ask_city_name(&mut city_name);
    let city_details_json: Value = get_city_details_as_json(&mut city_name);
}

fn ask_city_name(city_name: &mut String) {
    loop {
        println!("Enter a city name: ");

        stdin().read_line(city_name).unwrap();
        city_name.pop();

        let valid_input = is_string_alphabetic(city_name.to_string());
        if valid_input {
            break;
        }

        println!("Invalid city name '{}'!\n", city_name);
        city_name.clear();
    }
}

fn get_city_details_as_json(city_name: &mut String) -> Value {
    let api_url: String = format!("{}{}{}", "https://geocoding-api.open-meteo.com/v1/search?name=",
                                            CITY_NAME_PLACEHOLDER,
                                            "&count=1&language=en&format=json");

    let request_url: String = api_url.to_string().replace(CITY_NAME_PLACEHOLDER, &city_name.to_owned());
    let city_details: String = reqwest::blocking::get(request_url).unwrap().text().expect("Failed to get payload");
    let city_details_json: Value =  serde_json::from_str(city_details.as_str()).unwrap();
    return city_details_json;
}

fn is_string_alphabetic(check_str: String) -> bool {
    for c in check_str.chars() {
        if c.is_numeric() {
            return false;
        }
    }
    return true;
}
