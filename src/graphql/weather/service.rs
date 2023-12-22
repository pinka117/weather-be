use crate::graphql::Coordinate;
use serde::Deserialize;
use std::{env, error::Error};

#[derive(Deserialize, Debug)]
struct CurrentWeather {
    weather: Vec<Weather>,
}

#[derive(Deserialize, Debug)]
struct Weather {
    main: String,
}

pub async fn location_weather(coordinate: Coordinate) -> Result<String, Box<dyn Error>> {
    let api_key = match env::var_os("API_KEY") {
        Some(v) => v.into_string().unwrap(),
        None => panic!("$API_KEY is not set"),
    };
    let request_url = format!(
        "https://api.openweathermap.org/data/2.5/weather?lat={lat}&lon={lon}&appid={key}",
        lat = coordinate.latitude,
        lon = coordinate.longitude,
        key = api_key
    );
    let response = match reqwest::get(&request_url).await {
        Ok(response) => response,
        Err(_error) => return Err("Api failed")?,
    };

    let mut weather: CurrentWeather = match response.json().await {
        Ok(response) => response,
        Err(_error) => return Err("Api failed to parse json")?,
    };

    if weather.weather.get(0).is_none() {
        return Err("Invalid weather")?;
    } else {
        Ok(weather.weather.swap_remove(0).main)
    }
}
