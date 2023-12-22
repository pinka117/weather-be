use crate::graphql::Coordinate;
use serde::Deserialize;
#[derive(Deserialize)]
#[derive(Debug)]
struct WeatherApiResponse {
   current:CurrentWeather
}

#[derive(Deserialize)]
#[derive(Debug)]
struct CurrentWeather {
   weather:Vec<Weather>
}

#[derive(Deserialize)]
#[derive(Debug)]
struct Weather  {
   main:String
}

pub async fn location_weather(coordinate:Coordinate) -> Result<String, reqwest::Error> {
        let request_url = format!("https://api.openweathermap.org/data/3.0/onecall?lat={lat}&lon={lon}&exclude={part}&appid={key}",
                              lat = coordinate.latitude,
                              lon = coordinate.longitude,
                              key = "");
    println!("{}", request_url);
   
    let response = reqwest::get(&request_url).await?;
    
    let weather: WeatherApiResponse = response.json().await?;
    println!("{:?}", weather);

 Ok(weather.current.weather[0].main)
    }