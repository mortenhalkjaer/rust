
pub mod which {
//    use std::string;
    use std::env;
//    use std::fs;
    use std::path;
    
//  use std::{ascii::AsciiExt, f32::consts::E};

  fn starts_with(s: &String, substr: &str) -> bool {
    if substr.len() > s.len() { return false; }
    
    for (sub, main) in substr.chars().zip(s.chars()) {
      if !sub.eq_ignore_ascii_case(&main) {
        return false;
      }
    }
    return true;
  }

  fn ends_with(s: &String, substr: &str) -> bool {
    if substr.len() > s.len() { return false; }
    return true;
  }

  fn search(directory: &str, search_for: &String, extensions: &Vec<&str>) -> Option<String> {
    let b : bool = path::Path::new(directory).is_dir() && directory.len()>0;
    
    if b {
      for file in std::fs::read_dir(directory).unwrap() {
        let filename = file.unwrap().file_name().into_string().unwrap();

          //let ext = String::from(&filename[filename.len()-4..]).to_ascii_lowercase();
          //let ext = &filename[filename.len()-4..];
          //let name = &filename[0..filename.len()-4];

          for search_ext in extensions {
            let s = *search_ext;

            if filename.len() == s.len() + search_for.len() {
              if starts_with(&filename, search_for) &&
                ends_with(&filename, search_for) {
                  return Some(filename);
                }
            }
          }

      }
    };
    None
  }

  pub fn run() {
    // todo : check if path not defined
    let path = env::var("PATH").unwrap();
    let pathext = env::var("PATHEXT").unwrap();
    let filename: String = String::from("fcw");
    // todo : get filename from command line
    // todo: avoid duplicated code - use chain
    // todo : get extensions from pathext

    let extensions: Vec<&str> = pathext.split(";").collect();

    search(".\\", &filename, &extensions)
      .and_then(|f|{ println!("Found: {}{}", ".\\", f); Some(f) } );

    for dir in path.split(";") {
      search(dir, &filename, &extensions)
        .and_then(|f|{ println!("Found: {}{}", dir, f); Some(f) } );
    }
  }
}