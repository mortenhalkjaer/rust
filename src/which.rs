
pub mod which {
    use std::env;
    use std::fs;
    use std::io;
  
    use std::str;
    use std::iter;
    
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

  fn search(directory: &str, search_for: &String, extensions: &Vec<&str>) -> io::Result<Option<String>> {
    let s =
    fs::read_dir(directory)?.into_iter()
      .filter_map(|file|{
        let filename = file.unwrap().file_name().into_string().unwrap();
        if filename.len() >= search_for.len() { return Some(filename); } else { return None; }
      })
      .find(|filename| {
        let expected_length = filename.len() - search_for.len();

        extensions.into_iter()
            .filter(|ext| { ext.len() == expected_length })
            .any(|ext| { starts_with(&filename, search_for) && ends_with(&filename, ext) })
      });

    return Ok(s);
  }

  pub fn run() {    
    let path = env::var("PATH").expect("Missing PATH");
    let pathext = env::var("PATHEXT").expect("Missing PATHEXT");
    let filename: String = String::from("xcopy");
    // todo : get filename from command line
    let extensions: Vec<&str> = pathext.split(";").collect();
 
    for dir in iter::once(".\\").chain(path.split(";")) {
      match search(dir, &filename, &extensions) {
        Ok(filename) => {
          match filename {
            Some(f) => println!("Found: {}\\{}", dir, f),
            None => {}
          }
        },
        Err(err) => { println!("Unable to scan {}: {} ", dir, err); }
      };
    };
  }
}