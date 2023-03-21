use crate::{Cost, Path};
use std::io::{self, Write};

pub fn read_line(prompt: &str) -> String {
    let mut response = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut response).unwrap();
    response.trim().to_string()
}

pub fn display<C: Cost>(path: Option<Path<C>>) {
    if let Some(path) = path {
        println!("{}", path);
        eprintln!("{}", path.metrics());
    } else {
        println!("Podano błędne dane!");
    }
}
