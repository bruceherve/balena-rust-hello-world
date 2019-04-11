use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};
use reqwest;
use uname;

fn main() {
    let matches = App::new(crate_name!())
        .author(crate_authors!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(Arg::with_name("name").index(1).required(true))
        .get_matches();

    let info = uname::uname().unwrap();
    println!("{:?}", info);

    let response = reqwest::get(matches.value_of("name").unwrap()).and_then(|mut x| x.text());

    match response {
        Ok(x) => println!("OK: {}", x),
        Err(e) => println!("Failed: {}", e),
    };
}
