use async_graphql::Object;
pub mod weather;
pub use weather::*;

pub struct Query;

#[Object]
impl Query {
    async fn howdy(&self) -> &'static str {
        howdy()
    }
}