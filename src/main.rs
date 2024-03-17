use std::collections::HashMap;
mod markdown;
mod book;

fn main() {
    let mut categories: HashMap<String, Vec<String>> = HashMap::new();
    markdown::discover(&mut categories);
    book::add(&categories);
}
