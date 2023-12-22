use async_graphql::*;
pub mod weather;
pub use weather::*;
pub struct Query;

#[derive(InputObject)]
pub struct Coordinate {
    latitude: f64,
    longitude: f64,
}

#[Object]
impl Query {
    async fn location_weather(
        &self,
        coordinate: Coordinate,
    ) -> Result<String, async_graphql::Error> {
        match location_weather(coordinate).await {
            Ok(n) => Ok(n),
            Err(e) => Err(Error::new(e.to_string())),
        }
    }
}
