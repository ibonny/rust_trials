use std::process;

use jsonpath::Selector;
use reqwest;
use serde_json::Value;

trait PassThrough<T, E> {
    fn pass_through(self, msg: &str) -> T;
}

impl<T, E> PassThrough<T, E> for Result<T, E> where E: std::fmt::Debug {
    fn pass_through(self, msg: &str) -> T {
        match self {
            Ok(value) => value,
            Err(e) => {
                println!("{}: {:?}", msg, e);

                process::exit(1);
            }
        }
    }
}

async fn get_weather() -> Result<String, reqwest::Error> {
    let full_url = "http://api.weatherapi.com/v1/current.json?key=2af3e3b9db974c8bb66213204251405&q=Valrico&aqi=no";

    let result = reqwest::get(full_url)
        .await?
        .text()
        .await?;

    Ok(result)
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let weather = get_weather().await.pass_through("Unable to retrieve weather for location.");

    let json_weather: Value = serde_json::from_str(weather.as_str()).unwrap();
        // .unwrap_or_else(|e| {
        //     println!("Cannot deserialize result: {}", e);

        //     process::exit(1);
        // });
        
    let selector = Selector::new("$.location.localtime").unwrap();

    let local_time: &str = selector.find(&json_weather).first();

    println!("{:?}", local_time);

    Ok(())
}
