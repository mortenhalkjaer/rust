
pub mod which {
    use std::env;
    use std::fs;
    use std::io;
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
    // todo : check if path not defined
    let path = env::var("PATH").unwrap();
    let pathext = env::var("PATHEXT").unwrap();
    let filename: String = String::from("xcopy");
    // todo : get filename from command line

    let current : Vec<&str> = vec!(".\\");
    let paths : Vec<&str> = path.split(";").collect();

    let extensions: Vec<&str> = pathext.split(";").collect();

    for dir in current.iter().chain(paths.iter()) {
      search(dir, &filename, &extensions)
        .and_then(|f|{
          Ok(f.and_then(|g | { println!("Found: {}\\{}", dir, g); Some(g) } ))
         }
        );
    }
  }
}