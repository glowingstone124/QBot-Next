use std::env;
use dotenv::dotenv;
use once_cell::sync::Lazy;

pub fn read(input: &str) -> String {
    dotenv().ok();
    env::var(input).expect(&format!("{} not set", input)) // 获取环境变量
}

pub static API_ENDPOINT: Lazy<String> = Lazy::new(|| read("API_ENDPOINT"));
pub static ROCK_ENDPOINT: Lazy<String> = Lazy::new(|| read("ROCK_ENDPOINT"));
pub static ROCK_TOKEN: Lazy<String> = Lazy::new(|| read("ROCK_TOKEN"));