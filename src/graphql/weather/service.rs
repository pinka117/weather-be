use crate::graphql::Coordinate;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct CurrentWeather {
    weather: Vec<Weather>,
}

#[derive(Deserialize, Debug)]
struct Weather {
    main: String,
}

pub async fn location_weather(coordinate: Coordinate) -> Result<Option<String>, reqwest::Error> {
    let request_url = format!(
        "https://api.openweathermap.org/data/2.5/weather?lat={lat}&lon={lon}&appid={key}",
        lat = coordinate.latitude,
        lon = coordinate.longitude,
        key = ""
    );
    println!("{}", request_url);

    let response = reqwest::get(&request_url).await?;

    let mut weather: CurrentWeather = response.json().await?;
    println!("{:?}", weather);
    let res = if weather.weather.get(0).is_none() {
        None
    } else {
        Some(weather.weather.swap_remove(0).main)
    };

    Ok(res)
}
