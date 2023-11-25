use anyhow::Context;
use axum::extract::Query;

use crate::domain::{
    AppError, GeoResponse, LatLong, WeatherDisplay, WeatherQuery, WeatherResponse,
};

async fn fetch_lat_long(city: &str) -> Result<LatLong, anyhow::Error> {
    let endpoint = format!(
        "https://geocoding-api.open-meteo.com/v1/search?name={}&count=1&language=en&format=json",
        city
    );

    reqwest::get(&endpoint)
        .await?
        .json::<GeoResponse>()
        .await?
        .results
        .get(0)
        .cloned()
        .context("No results found")
}

async fn fetch_weather(lat_long: LatLong) -> Result<WeatherResponse, anyhow::Error> {
    let endpoint = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&hourly=temperature_2m",
        lat_long.latitude, lat_long.longitude
    );
    Ok(reqwest::get(&endpoint)
        .await?
        .json::<WeatherResponse>()
        .await?)
}

pub async fn weather_request(Query(params): Query<WeatherQuery>) -> Result<String, AppError> {
    let lat_long = fetch_lat_long(&params.city).await?;

    Ok(format!(
        "{:?}",
        WeatherDisplay::new(params.city, fetch_weather(lat_long).await?)
    ))
}
