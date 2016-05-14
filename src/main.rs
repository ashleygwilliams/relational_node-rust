use std::env;
use std::fs::File;

fn main() {
  let filename: String = match env::args().nth(1) {
    Some(filename)  => filename,
    None            => {
      println!("No filename passed. Please pass a filename using `cargo run -- <filename>`.");
      return; 
    },
  };

  
  match File::open(&filename) {
    Ok(data)  => data,
    Err(err)  => {
      println!("ERR: {}.", err);
      return;
    },
  };

  println!("Parsing {}.", filename);
}
