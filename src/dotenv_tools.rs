use std::env;
use dotenv::dotenv;
pub fn read(input: &str) -> String {
    dotenv().ok();
    env::var(input).expect(&format!("{} not set", input))
}
pub const API_ENDPOINT: &str = &*read("API_ENDPOINT");
pub const ROCK_ENDPOINT: &str = &*read("ROCK_ENDPOINT");