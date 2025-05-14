use std::{process, str::FromStr};

use jsonpath::Selector;
use reqwest;
use serde_json::{Error, Value};

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

fn find_json<'a>(root: &'a Value, path: &'a str) -> &'a str {
    let selector = Selector::new(path);

    let selector = match selector {
        Ok(val) => val,
        Err(e) => {
            println!("Unable to create new selector: {}", e);

            process::exit(1);
        }
    };

    let mut iter = selector.find(root);

    let nth = iter.nth(0);

    let nth = match nth {
        Some(v) => v,
        None => {
            println!("No matches found: {}", path);

            process::exit(0);
        },
    };

    let val = match nth.as_str() {
        Some(v) => v,
        None => {
            println!("Cannot decompose value: {}", path);

            process::exit(0);
        },
    };

    val
}

fn find_num<'a, T: FromStr>(root: &'a Value, path: &'a str) -> T {
    let selector = Selector::new(path);

    let selector = match selector {
        Ok(val) => val,
        Err(e) => {
            println!("Unable to create new selector: {}", e);

            process::exit(1);
        }
    };

    let mut iter = selector.find(root);

    let nth = iter.nth(0);

    let nth = match nth {
        Some(v) => v,
        None => {
            println!("No matches found: {}", path);

            process::exit(0);
        },
    };

    let ret_val = match nth.as_f64() {
        Some(v) => v,
        None => {
            println!("Cannot decompose value: {}", path);

            process::exit(0);
        },
    };

    let new_val = format!("{}", ret_val);

    let new_val = match new_val.parse::<T>() {
        Ok(n) => Ok(n),
        e => Err(e)
    };

    match new_val {
        Ok(n) => n,
        Err(_) => {
            println!("Error decomposing value: {}", path);

            process::exit(1);
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let weather = get_weather().await.pass_through("Unable to retrieve weather for location.");

    let json_weather: Result<Value, Error> = serde_json::from_str(weather.as_str());

    let json_weather = match json_weather {
        Ok(v) => v,
        Err(e) => {
            println!("Unable to deserialize value: {}", e);

            process::exit(1);
        }
    };

    // for key in json_weather.as_object().unwrap().get("current").unwrap().as_object().unwrap() {
    //     println!("{:?}", key);
    // }

    let local_time = find_json(&json_weather, "$.location.localtime");
    let last_updated = find_json(&json_weather, "$.current.last_updated");
    let temp_f = find_num::<f64>(&json_weather, "$.current.temp_f");
    let wind_mph = find_num::<f64>(&json_weather, "$.current.wind_mph");
    let windchill_f = find_num::<f64>(&json_weather, "$.current.windchill_f");
    let wind_dir = find_json(&json_weather, "$.current.wind_dir");
    let vis_miles = find_num::<f64>(&json_weather, "$.current.vis_miles");
    let pressure_mb = find_num::<f64>(&json_weather, "$.current.pressure_mb");
    let precip_in = find_num::<f64>(&json_weather, "$.current.precip_in");
    let heat_index_f = find_num::<f64>(&json_weather, "$.current.heatindex_f");
    let feelslike_f = find_num::<f64>(&json_weather, "$.current.feelslike_f");
    let dewpoint_f = find_num::<f64>(&json_weather, "$.current.dewpoint_f");
    let gust_mph = find_num::<f64>(&json_weather, "$.current.gust_mph");
    let humidity = find_num::<f64>(&json_weather, "$.current.humidity");
    let cloud_cover = find_num::<f32>(&json_weather, "$.current.cloud");

    println!("Local time: {}", local_time);
    println!("Last Updated: {}", last_updated);
    println!("Wind (MPH): {:.1}", wind_mph);
    println!("Wind Direction: {}", wind_dir);
    println!("Wind Gusts (MPH): {:.1}", gust_mph);
    println!("Temperature (F): {:.1}", temp_f);
    println!("WindChill (F): {:.1}", windchill_f);
    println!("Heat Index (F): {:.1}", heat_index_f);
    println!("Feels Like (F): {:.1}", feelslike_f);
    println!("Dew Point (F): {:.1}", dewpoint_f);
    println!("Visibility (Miles): {:.1}", vis_miles);
    println!("Pressure (mb): {:.1}", pressure_mb);
    println!("Precipitation (in): {:.1}", precip_in);
    println!("Humdity: {:.1}%", humidity);
    println!("Cloud Cover: {:.1}%", cloud_cover);

    Ok(())
}
