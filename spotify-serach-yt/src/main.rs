use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::{Serialize, Deserialize};
use std::env;
use dotenv::dotenv;

#[derive(Serialize, Deserialize, Debug)]
struct ExternalUrls {
    spotify: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Artist {
    name: String,
    external_urls: ExternalUrls,
}

#[derive(Serialize, Deserialize, Debug)]
struct Album {
    name: String,
    artists: Vec<Artist>,
    external_urls: ExternalUrls,
}

#[derive(Serialize, Deserialize, Debug)]
struct Track {
    name: String,
    href: String,
    popularity: u32,
    album: Album,
    external_urls: ExternalUrls,
}

#[derive(Serialize, Deserialize, Debug)]
struct APIResponse {
    tracks: Items<Track>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Items<T> {
    items: Vec<T>
}

fn print_tracks(tracks: Vec<&Track>) {
    for track in tracks {
        println!("{}", track.name);
        println!("{}", track.album.name);
        println!("{}", track.album.artists.iter().map(|artist| artist.name.to_string()).collect::<String>());
        println!("{}", track.external_urls.spotify);
        println!("{}", track.popularity);
        println!("-------------------");
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    let args: Vec<String> = env::args().collect();

    let search_query = &args[1];
    let access_token = env::var("SPOTIFY_ACCESS_TOKEN").expect("SPOTIFY_ACCESS_TOKEN must be set");

    let url = format!(
        "https://api.spotify.com/v1/search?q={query}&type=track",
        query=search_query,
    );

    let client = reqwest::Client::new();

    let response = client.get(url)
        .header(AUTHORIZATION, format!("Bearer {token}", token=access_token))
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await
        .unwrap();
    
    let status = response.status();
    let body = response.text().await.unwrap();

    match status {
        reqwest::StatusCode::OK => {
            match serde_json::from_str::<APIResponse>(&body) { // what happens if i don't use await? does reqwest always have to be used in a tokio runtime?
                Ok(parsed) => print_tracks(parsed.tracks.items.iter().collect()), // does iter take ownership?
                Err(_) => println!("the response did not match the struct")
            };

        },
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("Need to grab a new token");
        },
        other => {
            panic!("Something unexpected happened: {:?}", other);
        }
    };

}