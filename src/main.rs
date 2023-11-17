use scraper::{Html, Selector};
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    let file_content =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let document = Html::parse_document(&file_content);
}
