mod ascii;
mod xkcd;

use image::FilterType;
use reqwest::StatusCode;
use std::{env, error::Error, process, result};

type Result<T> = result::Result<T, Box<dyn Error>>;

fn run(num: Option<u16>) -> Result<()> {
    let contents = match num {
        Some(num) => xkcd::download(num)?,
        None => xkcd::download_random()?,
    };

    let image = image::load_from_memory(&contents)?;

    let width = term_size::dimensions().map_or(80, |(width, _)| width);

    let image = image.resize(width as u32, u32::max_value(), FilterType::Lanczos3);

    print!("{}", ascii::to_string(&image));

    Ok(())
}

fn main() {
    match env::args().nth(1).map(|arg| arg.parse()).transpose() {
        Ok(num) => {
            if let Err(err) = run(num) {
                match err.downcast::<reqwest::Error>() {
                    Ok(err) => {
                        if err.status() == Some(StatusCode::NOT_FOUND) {
                            eprintln!("error: invalid comic number: {}", err);
                            process::exit(1);
                        } else {
                            eprintln!("error retrieving image: {}", err);
                            process::exit(1);
                        }
                    }
                    Err(err) => {
                        eprintln!("error retrieving image: {}", err);
                        process::exit(1);
                    }
                }
            }
        }
        Err(err) => {
            eprintln!("error: invalid comic number: {}", err);
            process::exit(1);
        }
    }
}
