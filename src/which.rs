pub mod which {
    use std::{env, fs, io, str, iter};

    // todo  :make as extensions to String
  pub fn starts_with(s: &String, substr: &str) -> bool {        
    substr.chars().zip(s.chars())
      .all(|(sub,main)| sub.eq_ignore_ascii_case(&main))
  }

  fn ends_with(s: &String, substr: &str) -> bool {
    substr.chars().rev().zip(s.chars().rev())
      .all(|(sub, main)| sub.eq_ignore_ascii_case(&main))
  }

  fn search(directory: &str, search_for: &String, extensions: &Vec<&str>) -> io::Result<Option<String>> {    
    Ok(fs::read_dir(directory)?.into_iter()
      .map(|file| file.unwrap().file_name().into_string().unwrap())
      .find(|filename| 
        extensions.into_iter()           
            .any(|ext| { starts_with(&filename, search_for) && ends_with(&filename, ext) })
      ))
  }

  pub fn run() {    
    let path = env::var("PATH").expect("Missing PATH");    
    let filename: String = String::from("xcopy");
    // todo : get filename from command line     
    let pathext = env::var("PATHEXT").expect("Missing PATHEXT");
    let extensions: Vec<&str> = pathext.split(";").collect();
    



/* 
    find way to replace result check and option handling

    for dir in iter::once(".\\").chain(path.split(";")) {
      search(dir, &filename, &extensions).expect(msg)
        Ok(filename) => {
          match filename {
            Some(f) => println!("Found: {}\\{}", dir, f),
            None => {}
          }
        },
        Err(err) => { println!("Unable to scan {}: {} ", dir, err); }
      };
    }; 
*/
 
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