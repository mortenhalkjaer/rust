use std::env;
use std::fs;
use std::path;

pub mod which {
    use std::string;

//  use std::{ascii::AsciiExt, f32::consts::E};

  fn starts_with(s: String, substr: &str) -> bool {
    if substr.len() > s.len() { return false; }
    return true;
  }

  fn ends_with(s: String, substr: &str) -> bool {
    if substr.len() > s.len() { return false; }
    return true;
  }

  fn search(directory: &str, search_for: &String, extensions: &Vec<&str>) -> Option<String> {
    let b : bool = std::path::Path::new(directory).is_dir();

    // todo : skip emtpy paths

    if b {
      for file in std::fs::read_dir(directory).unwrap() {
        let filename = file.unwrap().file_name().into_string().unwrap();

          //let ext = String::from(&filename[filename.len()-4..]).to_ascii_lowercase();
          //let ext = &filename[filename.len()-4..];
          //let name = &filename[0..filename.len()-4];

          if starts_with(filename, search_for) {}

          for search_ext in extensions {
            let s = *search_ext;

            if filename.len() > s.len() {
              let ext = &filename[filename.len()-s.len()-1..];

              // todo : check for file name
              if s.eq_ignore_ascii_case(ext) {
                return Some(filename);
              }
            }
          }

      }
    } else {
      println!("Invalid directory {}\n", directory);
    }
    None
  }

  pub fn run() {
    // todo : check if path not defined
    let path = std::env::var("PATH").unwrap();
    let pathext = std::env::var("PATHEXT").unwrap();
    let filename: String = String::from("fcw");
    // todo : get filename from command line
    // todo: avoid duplicated code - use chain
    // todo : get extensions from pathext

    let extensions: Vec<&str> = pathext.split(";").collect();

    search(".\\", &filename, &extensions)
      .and_then(|f|{ println!("{}\\{}", ".\\", f); Some(f) } );

    for dir in path.split(";") {
      search(dir, &filename, &extensions)
        .and_then(|f|{ println!("{}\\{}", dir, f); Some(f) } );
    }
  }
}