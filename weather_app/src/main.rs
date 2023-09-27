use std::io::stdin;

fn main() {
    let mut city_name: String = String::new();

    ask_city_name(&mut city_name);
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
