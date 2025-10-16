use clap::{App, Arg};
use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Price {
    result: PriceResult,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct PriceResult {
    XXBTZEUR: XXBTZEUR,
}

#[derive(Deserialize, Debug)]
struct XXBTZEUR {
    c: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let matches = App::new("Sats Converter")
        .version("1.0")
        .author("Paulo Cunha <paulomrcunha@gmail.com>")
        .about("Converts sats to euros and euros to sats.")
        .arg(
            Arg::with_name("input currency")
                .short("c")
                .long("currency")
                .takes_value(true)
                .possible_values(&["sats", "eur"])
                .help("Select if your input is sats or euros"),
        )
        .arg(
            Arg::with_name("value")
            .help("The value to convert")
        )
        .get_matches();
    
    let currency = matches.value_of("input currency").unwrap_or("sats");
    let input = matches.value_of("value").unwrap();
    let input_nr = input.parse::<f32>().unwrap();

    let request_url = format!("https://api.kraken.com/0/public/Ticker?pair=BTCEUR");
    let response = reqwest::get(&request_url).await?;

    let prices: Price = response.json().await?;
    let btceur_price = prices.result.XXBTZEUR.c[0].parse::<f32>().unwrap();

    let result = match currency {
        "sats" => format!("{} sats are {} euros.", input_nr, input_nr * btceur_price / 100000000f32),
        "eur" => format!("{} euros are {} sats.", input_nr, input_nr / btceur_price * 100000000f32),
        _ => panic!("Invalid code flow - unsuported currency")
    };

    println!("{}", result);
    println!("Current bitcoin price is {} euros.", btceur_price);

    Ok(())
}
