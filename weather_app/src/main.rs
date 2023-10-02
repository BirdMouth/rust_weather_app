use serde_json::Value;
use std::io::stdin;

fn main() {
    let mut city_name: String = String::new();

    ask_city_name(&mut city_name);
    let city_details_json: Value = get_city_details_as_json(&mut city_name);
    let city_lat_lon: (String, String) = get_city_lat_lon(&city_details_json);
    let weather_details: Value = get_city_weather_from_lat_lon(city_lat_lon.0, city_lat_lon.1);
    println!("It is {} celsius in {}, WMO code is {}.", weather_details["current_weather"]["temperature"],
                                                        city_name,
                                                        weather_details["current_weather"]["weathercode"]);
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
                                            city_name,
                                            "&count=1&language=en&format=json");

    let city_details: String = reqwest::blocking::get(api_url).unwrap().text().expect("Failed to get payload");
    let city_details_json: Value =  serde_json::from_str(city_details.as_str()).unwrap();
    return city_details_json;
}

fn get_city_lat_lon(city_details_json: &Value) -> (String, String) {
    let city_lat: String = city_details_json["results"][0]["latitude"].to_string();
    let city_lon: String = city_details_json["results"][0]["longitude"].to_string();
    return (city_lat, city_lon);
}

fn get_city_weather_from_lat_lon(lat: String, lon: String) -> Value {
    let request_url: String = format!("{}{}{}{}{}", "https://api.open-meteo.com/v1/forecast?latitude=",
                                                    lat,
                                                    "&longitude=",
                                                    lon,
                                                    "&current_weather=true");
    let weather: String = reqwest::blocking::get(request_url).unwrap().text().expect("Failed to get payload");
    let weather_json: Value =  serde_json::from_str(weather.as_str()).unwrap();
    return weather_json;
}

fn is_string_alphabetic(check_str: String) -> bool {
    for c in check_str.chars() {
        if c.is_numeric() {
            return false;
        }
    }
    return true;
}
