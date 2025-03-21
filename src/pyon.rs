//! Bunny processing functions
//! 
//! This module provides functions to:
//! - Load a list of bunnypaths from the `bunnies` directory
//! - Pick a random path out of the list
//! - Load the data from a certain bunnyfile
//! - Print the bunnyfile data and the literal string

use include_dir::{include_dir, Dir};
use rand::seq::SliceRandom;

static BUNNY_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/bunnies");

/// Load a vector of bunny paths from the `BUNNY_DIR` folder
fn load_bunnies() -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    let glob = "*.txt";

    for path in BUNNY_DIR.find(glob).unwrap() {
        result.push(path.path().display().to_string());
    }

    result
}

/// Pick a bunny path from a vector of bunny paths
/// 
/// # Arguments
/// 
/// * `options` - A vector containing string paths to bunnyfiles
/// 
/// # Returns
/// 
/// One of the strings in `options`, randomly picked
fn pick_bunny(options: &Vec<String>) -> String {
    let mut rng = rand::thread_rng();
    let result = options.choose(&mut rng);

    match result {
        None => "Pyon".to_string(),
        Some(path) => path.to_string(),
    }
}

/// Load data from a bunnyfile
/// 
/// # Arguments
/// 
/// * `path` - A string path to a bunnyfile
/// 
/// # Returns
/// 
/// A string containing the contents of the bunnyfile
fn load_bunny(path: &String) -> String {
    let file = BUNNY_DIR.get_file(path).unwrap();
    let contents = file.contents_utf8().unwrap().to_string();
    contents
}

/// Print bunny data to the screen
/// 
/// # Arguments
/// 
/// * `bunny` - bunny data
fn print_bunny(bunny: &String) {
    println!("{bunny}");
}

/// Print the literal string
/// 
/// # Arguments
/// 
/// * `literally` - A boolean, whether to print the literal string or not
fn print_literal(literally: bool){
    if literally {
        println!("I am LITERALLY this bunny:");
    }
}

/// Print a certain number of randomly picked bunnies with a literal string
/// 
/// # Arguments
/// 
/// * `literally` - A boolean, whether to print the literal string or not
/// * `count` - How many bunnies to print
pub fn print_bunnies(literally: bool, count: u16) {
    let bunny_paths = load_bunnies();

    for _ in 0..count {
        print_literal(literally);
        let bunny_path = pick_bunny(&bunny_paths);
        let bunny_art = load_bunny(&bunny_path);
        print_bunny(&bunny_art);
    }
}
