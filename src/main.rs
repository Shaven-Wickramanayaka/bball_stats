use serde::Deserialize;
use reqwest::header::AUTHORIZATION;
use chrono::{DateTime,Utc};
use std::time::SystemTime;
use std::collections::HashMap;
use std::io::{stdin,stdout,Write};

#[derive(Deserialize)]

struct AllData{
    data: Vec<Stats>
}
#[derive(Deserialize)]
struct Stats{
    home_team: HomeTeam,
    visitor_team: VisitorTeam,
    home_team_score: u64,
    visitor_team_score:u64,
    status_state: String,
}
#[derive(Deserialize)]
struct HomeTeam{
    full_name: String,
    abbreviation: String,
}
#[derive(Deserialize)]
struct VisitorTeam{
    full_name: String,
    abbreviation: String,
}
fn main() {
    println!("NBA Games Today\n");
    let url = format!("https://api.balldontlie.io/nba/v1/games");
    let client = reqwest::blocking::Client::new();
    let now = Utc::now().format("%Y-%m-%d").to_string();
    let mut params = HashMap::new();
    params.insert("dates[]", now);
    let data: AllData = client.get(url).header(AUTHORIZATION, "65ab2ce7-ad41-43bf-a304-0f7ec82a392f").query(&params).send().expect("Error getting api").json().expect("Error parsing Json");

    for game in data.data{
            println!("{}(Home Team) vs {}(Away Team) | Status: {}", game.home_team.full_name, game.visitor_team.full_name, game.status_state);
            println!("{}:{} | {}:{}",game.home_team.abbreviation, game.home_team_score, game.visitor_team.abbreviation, game.visitor_team_score);
            println!("\n");
    }
    let mut s=String::new();
    print!("Press a key to exit");
    let _=stdout().flush();
    let _ = stdin().read_line(&mut s);
}
