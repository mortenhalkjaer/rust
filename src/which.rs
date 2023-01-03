use std::env;
use std::fs;

pub mod which {
    use std::ascii::AsciiExt;

  fn search(directory: &str, search_for: &String) -> Option<String> {
    for file in std::fs::read_dir(directory).unwrap() {
      let filename = file.unwrap().file_name().into_string().unwrap();
      if filename.len() > 3 {
        let ext = String::from(&filename[filename.len()-3..]).to_ascii_lowercase();
        let name = &filename[0..filename.len()-4];

        if (("exe" == ext) || ( "bat" == ext)) && search_for.eq_ignore_ascii_case(name) {
          return Some(filename)
        }
      }
    }
    None
  }

  pub fn run() {
    // todo : error handling - if empty/missing path
    // todo : push current directory into this one
    let path = std::env::var("PATH").unwrap();
    let filename: String = String::from("fcw");

    for dir in path.split(";") {
      // todo : path - with directory not found
      search(dir, &filename).and_then(|f|{ println!("{}\\{}", dir, f); Some(f) } );
    }
  }
}