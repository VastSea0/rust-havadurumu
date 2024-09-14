mod okul;

use std::io;
use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize)]
struct WeatherResponse {
    location: Location,
    current: Current,
}

#[derive(Deserialize)]
struct Location {
    name: String,
}

#[derive(Deserialize)]
struct Current {
    temp_c: f64,
    condition: Condition,
}

#[derive(Deserialize)]
struct Condition {
    text: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    loop {
        let mut country = String::new();
        println!("Enter your country name:");

        io::stdin()
            .read_line(&mut country)
            .expect("Failed to read line");


        let url = format!(
            "http://api.weatherapi.com/v1/current.json?key=3d453ad4134449b0807210757241409&q={}&aqi=no",
            country
        );

        let response = reqwest::get(url).await?;


        let weather: WeatherResponse = response.json().await?;

        println!("----------------------------------------------");
        println!("Location: {}", weather.location.name);
        println!("Temperature: {}°C", weather.current.temp_c);
        println!("Condition: {}", weather.current.condition.text);
        println!("----------------------------------------------");

    }

    Ok(())
}
