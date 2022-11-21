
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

fn main() {
    //let input_path = "~/git/rust/scrape_placra/src/config.json";
    let input_path = Path::new("./src/config.json");
    let output_path = Path::new("./src/output.json");

    let mut CFG = {
        let CFG = std::fs::read_to_string(&input_path).unwrap();
        serde_json::from_str::<CONFIG>(&CFG).unwrap()
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

    let mut out: Vec<Recomendation> = Default::default();
    //let mut day_dict =Dict<Vec<Recomendation>>::new();
    /*for i in 0..CFG.Houses.len() {
        out.push(
            Recomendation {
                house: String::clone(&CFG.Houses[i].name),
                rec: Default::default(),
            }
        )
    }*/
    for t in titles {
        let V: Vec<&str> = t.split("\n").collect();
        for v in V {
            if v.contains("<p>"){
                for i in 0..CFG.Houses.len() {
                    let mut this_house = String::clone(&CFG.Houses[i].name);
                    if v.contains(&this_house) {
                        let  rec = Recomendation {
                            house: String::clone(&this_house),
                            //rec: Default::default(),
                            stock: String::from("Stock"),
                            price: 32.0,
                            rec: String::from("String"),
                        };
                        out.push(rec);
                        /*for this in &out {
                            if this.house == this_house {
                                this.rec.push(rec)
                            }
                        }
                        */
                    }
                }
            }
        }
        std::fs::write(output_path,
        serde_json::to_string_pretty(&out).unwrap());
    }    
}
