use rand::Rng;
use reqwest::Client;
use serde::Deserialize;
use std::io::Read;

use crate::Result;

#[derive(Deserialize)]
struct Info {
    num: u16,
    img: String,
}

fn fetch(client: &Client, num: u16) -> Result<Vec<u8>> {
    let info: Info = client
        .get(&format!("https://xkcd.com/{}/info.0.json", num))
        .send()?
        .error_for_status()?
        .json()?;

    let mut buf = Vec::new();
    client.get(&info.img).send()?.read_to_end(&mut buf)?;

    Ok(buf)
}

pub fn download(num: u16) -> Result<Vec<u8>> {
    fetch(&Client::new(), num)
}

pub fn download_random() -> Result<Vec<u8>> {
    let client = Client::new();
    let info: Info = client.get("https://xkcd.com/info.0.json").send()?.json()?;

    fetch(&client, rand::thread_rng().gen_range(1, info.num + 1))
}
