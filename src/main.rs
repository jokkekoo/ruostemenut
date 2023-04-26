use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let string = String::from("error");
    let restaurant = std::env::args().nth(1).expect("no pattern given");

    let resp = reqwest::blocking::get("https://fi.jamix.cloud/apps/menuservice/rest/haku/menu/93077/49?lang=en")?
        .text()
        .unwrap_or(string);
    
    println!("{:#?}", resp);
    
    Ok(())
}
