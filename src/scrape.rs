
use serde_derive::{Deserialize, Serialize};
use chrono::{Datelike, NaiveDate, Weekday};
use std::path::Path;
use regex::Regex;

#[derive(Deserialize, Serialize, Debug)]
struct Recomendation {
    house: String,
    stock: String,
    price: f32,
    rec: String,
    raw: String,
}

impl Recomendation {
    fn new (raw: &str) -> Self {
        return Recomendation {
            house: String::from("Unknown"),//clone(&this_house),
            stock: String::from("Stock"),
            price: 100.0,
            rec: String::from("String"),
            raw: String::clone(&raw.to_string())
        };
    }
    fn from(raw: &str, cfg: &CONFIG) -> Self{
        for i in 0..cfg.houses.len() {
            let this_house = String::clone(&cfg.houses[i].name);
            if raw.contains(&this_house) {
                //let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
                println!("{:?}", &cfg.houses[i].regex[0].company[0].as_str());
                println!("{}",raw);
                let re = Regex::new(&cfg.houses[i].regex[0].company[0].as_str()).unwrap();
                let mut company;
                
                match re.captures(raw) {
                    Some(caps) => {
                        //company = caps.get(1).unwrap().as_str();
                        company = match caps.get(1) {
                            Some(internal) => {
                                internal.as_str()
                            }
                            None => {
                                "Unknown"
                            }
                        };
                        println!("{}", company);
                    }
                    None => {
                        company = "Unknown";// The regex did not match. Deal with it here!
                    }
                }
                /*
                if re.
                let caps = re.captures(raw).unwrap() {
                    company = caps.get(1).map_or("", |m| m.as_str());
                } else {
                    company = "Unknown";
                }*/

                return Recomendation {
                    house: String::clone(&this_house),
                    stock: String::from(company),
                    price: 32.0,
                    rec: String::from("String"),
                    raw: String::clone(&raw.to_string())
                };
            }
        }
        return Recomendation::new(raw);
    }
} 

#[derive(Deserialize, Serialize, Debug)]
struct REGEX {
    company: Vec<String>,
    buy: Vec<String>,
    sell: Vec<String>,
    neutral: Vec<String>
}
            
            
#[derive(Deserialize, Serialize, Debug)]
struct HOUSES {
    name: String,
    regex: Vec<REGEX>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CONFIG {    
    houses: Vec<HOUSES>,
    input_path: String,
    output_path: String,
    path_404: String
}


fn generate_adress(date: NaiveDate) -> Option<String> {
    /*
    https://www.placera.se/placera/redaktionellt/2023/02/20/mandagens-alla-ny-aktierekar.html
    https://www.placera.se/placera/redaktionellt/2023/01/24/tisdagens-alla-nya-aktierekar.html
    https://www.placera.se/placera/redaktionellt/2023/02/15/onsdagens-alla-nya-aktierekar.html
    https://www.placera.se/placera/redaktionellt/2023/02/16/torsdagens-alla-nya-aktierekar.html
    https://www.placera.se/placera/redaktionellt/2022/11/10/torsdagens-alla-ny-aktierekar.html
    https://www.placera.se/placera/redaktionellt/2023/02/17/fredagens-alla-nya-aktierekar.html
    */
    
    let mut start = String::from("https://www.placera.se/placera/redaktionellt/");
    
    let day = match date.weekday() {
        Weekday::Mon => String::from("mandagens"),
        Weekday::Tue => String::from("tisdagens"),
        Weekday::Wed => String::from("onsdagens"),
        Weekday::Thu => String::from("torsdagens"),
        Weekday::Fri => String::from("fredagens"),
        _ => {
            return None;
        }
    };
    let end = String::from("-alla-nya-aktierekar.html");
    let day_nr_str;
    let month_nr_str;
    if date.month() < 10 {
        month_nr_str = String::from("0".to_owned() + &date.month().to_string());
    } else {
        month_nr_str = date.month().to_string();
    }
    if date.day() < 10 {
        day_nr_str = String::from("0".to_owned() + &date.day().to_string());
    } else {
        day_nr_str = date.day().to_string();
    }
    let middle = String::from(date.year().to_string() + "/" + &month_nr_str.to_string() + "/" + &day_nr_str.to_string() + "/") + &day;
    start.push_str(&middle);
    start.push_str(&end);
    return Some(start); 
}

pub fn scrape_placera(cfg: &CONFIG) {
    let mut all_recomendations: Vec<Recomendation> = Default::default();
    
    let date = NaiveDate::from_ymd_opt(2022, 11, 09).unwrap();
    //let output_path = Path::new("./src/output.json");
    if let Some(adress) = generate_adress(date) {
        println!("{}",adress);
        if let Some(daily_recomendations) = parse_adress(&adress, &cfg) {
            for rec in daily_recomendations {
                all_recomendations.push(rec);
            }
        }
    }
    println!("{}",all_recomendations.len());
    let date = NaiveDate::from_ymd_opt(2022, 11, 10).unwrap();
    
    if let Some(adress) = generate_adress(date) {
        println!("{}",adress);
        if let Some(daily_recomendations) = parse_adress(&adress, &cfg) {
            for rec in daily_recomendations {
                all_recomendations.push(rec);
            }
        }
    }
    println!("{}",all_recomendations.len());
    all_recomendations.sort_by(|d1, d2| d1.house.cmp(&d2.house));
    match std::fs::write(&cfg.output_path, serde_json::to_string_pretty(&all_recomendations).unwrap()) {
        Err(e) => println!("{:?}", e),
        _ => ()
    }
}

fn parse_adress(adress: &String, cfg: &CONFIG) -> Option<Vec<Recomendation>>{
    let mut response = reqwest::blocking::get(adress)
        .unwrap()
        .text()
        .unwrap();
    if response.contains(&cfg.path_404) {
        let adress = adress.replace("nya-", "ny-");
        println!("{}",adress);
        response = reqwest::blocking::get(adress)
        .unwrap()
        .text()
        .unwrap();
    }
    
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

    return Some(out);
}