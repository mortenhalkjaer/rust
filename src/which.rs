use std::env;
use std::fs;
use std::path;

pub mod which {
    use std::ascii::AsciiExt;

  fn search(directory: &str, search_for: &String) -> Option<String> {
    let b : bool = std::path::Path::new(directory).is_dir();

    // todo : skip emtpy paths

    if b {
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
    } else {
      println!("Invalid directory {}\n", directory);
    }
    None
  }

  pub fn run() {
    let path = std::env::var("PATH").unwrap();
    let filename: String = String::from("fcw");
    // todo : get filename from command line
    // todo: avoid duplicated code
    search(".\\", &filename).and_then(|f|{ println!("{}\\{}", ".\\", f); Some(f) } );

    for dir in path.split(";") {
      search(dir, &filename).and_then(|f|{ println!("{}\\{}", dir, f); Some(f) } );
    }
  }
}