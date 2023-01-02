
use serde_derive::{Deserialize, Serialize};
use std::path::Path;
use std::fs;

#[derive(Deserialize, Serialize, Debug)]
struct Recomendation {
    house: String,
    //rec: Vec<house_day>,
    stock: String,
    price: f32,
    rec: String,
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
    Houses: Vec<HOUSES>,
}

//fn read_from_json(p: Path) -> Vec<Recomendation> {} TODO
//fn generate_

/*fn read_from_placera(p: String) -> std::iter::Map<I, F> {//Vec<Recomendation> {
    let response = reqwest::blocking::get(
        p,
        //"https://www.placera.se/placera/redaktionellt/2022/11/09/onsdagens-alla-ny-aktierekar.html",
    )
    .unwrap()
    .text()
    .unwrap();
    let document = scraper::Html::parse_document(&response);
    let title_selector = scraper::Selector::parse("div.parbase").unwrap();
    return document.select(&title_selector).map(|x| x.inner_html());
}*/

pub fn scrape_placera() {
    let input_path = Path::new("./src/config.json");
    let output_path = Path::new("./src/output.json");

    let mut cfg = {
        let cfg = std::fs::read_to_string(&input_path).unwrap();
        serde_json::from_str::<CONFIG>(&cfg).unwrap()
    };
    let response = reqwest::blocking::get(
        "https://www.placera.se/placera/redaktionellt/2022/11/09/onsdagens-alla-ny-aktierekar.html",
    )
    .unwrap()
    .text()
    .unwrap();
    let document = scraper::Html::parse_document(&response);
    let title_selector = scraper::Selector::parse("div.parbase").unwrap();
    let titles = document.select(&title_selector).map(|x| x.inner_html());
    
    //let ppp = String::from("https://www.placera.se/placera/redaktionellt/2022/11/09/onsdagens-alla-ny-aktierekar.html");
    //let titles = read_from_placera(ppp);
    let mut out: Vec<Recomendation> = Default::default();

    for t in titles {
        let V: Vec<&str> = t.split("\n").collect();
        for v in V {
            if v.contains("<p>"){
                let mut cont = false;
                for i in 0..cfg.Houses.len() {
                    let this_house = String::clone(&cfg.Houses[i].name);
                    if v.contains(&this_house) {
                        cont = true;
                        let  rec = Recomendation {
                            house: String::clone(&this_house),
                            stock: String::from("Stock"),
                            price: 32.0,
                            rec: String::from("String"),
                        };
                        out.push(rec);
                    }
                }
                if !cont {
                    println!("{v}");
                }
            }
        }
        std::fs::write(output_path,
        serde_json::to_string_pretty(&out).unwrap());
    }    
    println!("end of func");
}
