use std::{
    fs::File,
    io::{stdin, stdout, Write},
};

use reqwest::blocking::*;

fn main() {
    let path = std::env::args().nth(1).unwrap();

    let mut url = String::new();
    print!("input the url here: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut url).unwrap();

    let my_client = Client::new();
    let req = my_client.get(url.trim()).send().unwrap();
    let html = req.text().unwrap();

    let url_end = html.find("?param=130y130").unwrap();
    let url_start = html[..url_end].rfind("<img src=\"").unwrap() + "<img src=\"".len();
    let img_url: &str = &html[url_start..url_end];
    println!("{}", img_url);

    let name_start = html.find("所属专辑：").unwrap();
    let name_end = name_start + html[name_start..].find("</a>").unwrap();
    let name_start = name_start + html[name_start..name_end].find(">").unwrap() + ">".len();
    let name = &html[name_start..name_end];
    let name = name.replace("/", "-");
    println!("{}", name);

    let img = my_client.get(img_url).send().unwrap();
    let mut file = File::create(path + "/" + &name + ".jpg").expect("unable to create file");
    file.write_all(&img.bytes().unwrap()).unwrap();
}
