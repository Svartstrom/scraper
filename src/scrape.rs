
use serde_derive::{Deserialize, Serialize};
use chrono::{Datelike, NaiveDate, Weekday};
use std::path::Path;

#[derive(Deserialize, Serialize, Debug)]
struct Recomendation {
    house: String,
    stock: String,
    price: f32,
    rec: String,
    raw: String,
}

impl Recomendation {
    fn new () -> Self {
        return Recomendation {
            house: String::from("Unknown"),//clone(&this_house),
            stock: String::from("Stock"),
            price: 100.0,
            rec: String::from("String"),
            raw: String::from("v")
        };
    }
    fn from(v: &str, cfg: &CONFIG) -> Self{
        for i in 0..cfg.houses.len() {
            let this_house = String::clone(&cfg.houses[i].name);
            if v.contains(&this_house) {
                return Recomendation {
                    house: String::clone(&this_house),
                    stock: String::from("Stock"),
                    price: 32.0,
                    rec: String::from("String"),
                    raw: String::clone(&v.to_string())
                };
            }
        }
        return Recomendation::new();
    }
} 

#[derive(Deserialize, Serialize, Debug)]
struct HOUSES {
    name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CONFIG {    
    houses: Vec<HOUSES>,
    input_path: String,
    output_path: String
}


fn generate_adress(date: NaiveDate) -> Option<String> {
    let mut start = String::from("https://www.placera.se/placera/redaktionellt/");
    
    let day = match date.weekday() {
        Weekday::Mon => String::from("måndagens"),
        Weekday::Tue => String::from("tisdagens"),
        Weekday::Wed => String::from("onsdagens"),
        Weekday::Thu => String::from("torsdagens"),
        Weekday::Fri => String::from("fredagens"),
        _ => {
            return None;
        }
    };

    let end = String::from("-alla-ny-aktierekar.html");
    let day_nr_str;
    if date.day() < 10 {
        day_nr_str = String::from("0".to_owned() + &date.day().to_string());
    } else {
        day_nr_str = date.day().to_string();
    }
    let middle = String::from(date.year().to_string() + "/" + &date.month().to_string() + "/" + &day_nr_str.to_string() + "/") + &day;
    start.push_str(&middle);
    start.push_str(&end);
    return Some(start); 
}

pub fn scrape_placera(cfg: &CONFIG) {

    let date = NaiveDate::from_ymd_opt(2022, 11, 09).unwrap();
    //let output_path = Path::new("./src/output.json");
    let mut out: Vec<Recomendation> = Default::default();
    if let Some(adress) = generate_adress(date) {
        println!("{}",adress);
        out = parse_adress(&adress, &cfg);
    }
    match std::fs::write(&cfg.output_path, serde_json::to_string_pretty(&out).unwrap()) {
        Err(e) => println!("{:?}", e),
        _ => ()
    }
}

fn parse_adress(adress: &String, cfg: &CONFIG) -> Vec<Recomendation>{

    let response = reqwest::blocking::get(adress)
        .unwrap()
        .text()
        .unwrap();
    
    let document = scraper::Html::parse_document(&response);
    let title_selector = scraper::Selector::parse("div.parbase").unwrap();
    let titles = document.select(&title_selector).map(|x| x.inner_html());
    
    let mut out: Vec<Recomendation> = Default::default();

    for title in titles {
        let rows: Vec<&str> = title.split("\n").collect();
        for v in rows {
            if v.contains("<p>"){
                let rec = Recomendation::from(&v, &cfg);
                out.push(rec);
            }
        }
    }    
    out.sort_by(|d1, d2| d1.house.cmp(&d2.house));
    return out;
}