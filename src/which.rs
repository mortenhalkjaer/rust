
pub mod which {
    use std::env;
//    use std::fs;
    use std::path;
    use std::str;
    //use std::iter;

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
    for (sub, main) in substr.chars().rev().zip(s.chars().rev()) {
      if !sub.eq_ignore_ascii_case(&main) { return false; }
    }
    return true;
  }

  fn search(directory: &str, search_for: &String, extensions: &Vec<&str>) -> Option<String> {
    let b : bool = path::Path::new(directory).is_dir() && directory.len()>0;

    if b {
      for file in std::fs::read_dir(directory).unwrap() {
        let filename = file.unwrap().file_name().into_string().unwrap();

          for search_ext in extensions {
            let s = *search_ext;

            if filename.len() == s.len() + search_for.len() {
              if starts_with(&filename, search_for) {


                if ends_with(&filename, s) {
                  return Some(filename);
                }
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
    let filename: String = String::from("xcopy");
    // todo : get filename from command line
    // todo: avoid duplicated code - use chain

    let current : Vec<&str> = vec!(".\\");
    let paths : Vec<&str> = path.split(";").collect();

    let extensions: Vec<&str> = pathext.split(";").collect();

    for dir in current.iter().chain(paths.iter()) {
      search(dir, &filename, &extensions)
      .and_then(|f|{ println!("Found: {}{}", dir, f); Some(f) } );
    }
  }
}