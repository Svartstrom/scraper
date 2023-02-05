
use serde_derive::{Deserialize, Serialize};
use chrono::{Datelike, NaiveDate, Weekday};
use std::path::Path;

#[derive(Deserialize, Serialize, Debug)]
struct Recomendation {
    house: String,
    //rec: Vec<house_day>,
    stock: String,
    price: f32,
    rec: String,
    raw: String,
    //date: NaiveDate,
}

/*#[derive(Deserialize, Serialize, Debug)]
struct house_day {
    stock: String,
    price: f32,
    rec: String,
}*/

#[derive(Deserialize, Serialize, Debug)]
struct HOUSES {
    name: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct CONFIG {
    houses: Vec<HOUSES>,
}


fn generate_adress(date: NaiveDate) -> Option<String> {
    let mut start = String::from("https://www.placera.se/placera/redaktionellt/");
    
    let day = match date.weekday() {
        Weekday::Mon => String::from("måndagens"),
        Weekday::Tue => String::from("tisdagens"),
        Weekday::Wed => String::from("onsdagens"),
        Weekday::Thu => String::from("torsdagens"),
        Weekday::Fri => String::from("fredagens"),
        _ => String::from(""),
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

pub fn scrape_placera(){
    let input_path = Path::new("./src/config.json");
    let output_path = Path::new("./src/output.json");

    let cfg = {
        let cfg = std::fs::read_to_string(&input_path).unwrap();
        serde_json::from_str::<CONFIG>(&cfg).unwrap()
    };
    let date = NaiveDate::from_ymd_opt(2022, 11, 09).unwrap();
    
    if let Some(adress) = generate_adress(date) {
        //Some(adress) => adress,
        println!("{}",adress);
        let response = reqwest::blocking::get(adress)
        .unwrap()
        .text()
        .unwrap();
        let document = scraper::Html::parse_document(&response);
        let title_selector = scraper::Selector::parse("div.parbase").unwrap();
        let titles = document.select(&title_selector).map(|x| x.inner_html());
        /*let response = match reqwest::blocking::get(adress) {
            Ok(Value) => {
                let resp = match Value.text() {
                    Ok(Value) => {
                        return Value;
                    }
                    Err(error) => {
                        println!("{}", error);
                    }
                };
            }
            Err(error) => println!("{}", error),
        };*/

    };
}
/*
pub fn old_scrape_placera() {

    println!("{}",adress);
    let response = match reqwest::blocking::get(adress) {
        Ok(Value) => {
            let resp = match Value.text() {
                Ok(Value) => Value,
                Err(error) => println!("{}", error),
            };
        }
        Err(error) => println!("{}", error),
    };
    //    "https://www.placera.se/placera/redaktionellt/2022/11/09/onsdagens-alla-ny-aktierekar.html",
    //)
    //.unwrap()
    //.text()
    //.unwrap();

    let document = scraper::Html::parse_document(&response);
    let title_selector = scraper::Selector::parse("div.parbase").unwrap();
    let titles = document.select(&title_selector).map(|x| x.inner_html());
    
    let mut out: Vec<Recomendation> = Default::default();

    for title in titles {
        let rows: Vec<&str> = title.split("\n").collect();
        for v in rows {
            if v.contains("<p>"){
                let mut cont = false;
                for i in 0..cfg.houses.len() {
                    let this_house = String::clone(&cfg.houses[i].name);
                    if v.contains(&this_house) {
                        cont = true;
                        let  rec = Recomendation {
                            house: String::clone(&this_house),
                            stock: String::from("Stock"),
                            price: 32.0,
                            rec: String::from("String"),
                            raw: String::from(v)
                        };
                        out.push(rec);
                    }
                }
                if !cont {
                    println!("{v}");
                }
            }
        }
        match std::fs::write(output_path, serde_json::to_string_pretty(&out).unwrap()) {
            Err(e) => println!("{:?}", e),
            _ => ()
        }
    }    
    println!("end of func");
}
*/