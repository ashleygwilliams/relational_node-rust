use std::env;

fn main() {
  let filename: String = match env::args().nth(1) {
    Some(filename)  => filename,
    None               => {
      println!("No argument passed. Please pass a filename.");
      return; 
    },
  };

  println!("{}", filename);
}
