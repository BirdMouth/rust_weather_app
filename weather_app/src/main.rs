use std::io::stdin;

static CITY_NAME_PLACEHOLDER: &str = "[CITY_NAME]";

fn main() {
    let api_url: String = format!("{}{}{}", "https://geocoding-api.open-meteo.com/v1/search?name=",
                                            CITY_NAME_PLACEHOLDER,
                                            "&count=1&language=en&format=json");

    let mut city_name: String = String::new();
    ask_city_name(&mut city_name);

    let request_url: String = api_url.to_string().replace(CITY_NAME_PLACEHOLDER, &city_name.to_owned());
    let city_details = reqwest::blocking::get(request_url).unwrap().text();

    println!("{:#?}", city_details);

    // Todo:
    // Parse coordinates from city_details and use them to request city weather data from:
    // https://open-meteo.com/en/docs#latitude=65.0124&longitude=25.4682
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

fn is_string_alphabetic(check_str: String) -> bool {
    for c in check_str.chars() {
        if c.is_numeric() {
            return false;
        }
    }
    return true;
}
