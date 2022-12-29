use std::{
  fs,
  io::{prelude::*, BufReader},
  net::{TcpListener, TcpStream},
  };
use server::ThreadPool;


pub trait Draw {
  fn draw(&self);
}

pub struct Button {
   text: String,
}

impl Draw for Button {
  fn draw(&self) {
    print!("{}\n", self.text);
  }
}

pub struct TextBox {
  x: i32, 
  y: i32,
}

impl Draw for TextBox {
  fn draw(&self) {
    print!("textbox {}{}\n", self.x, self.y);
  }
}

struct UI {
  items: Vec<Box<dyn Draw>>
}

fn main() {
  let ui = UI {
    items: vec![
      Box::new(TextBox { x: 2, y: 3}),
      Box::new(Button { text: String::from("he") })
    ]
  };

  for x in ui.items.iter() {
    x.draw();
  }
  print!("done\n");
  
}

fn main1() {
  print!("Hej\n");
  let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
  let pool = ThreadPool::new(4);
  for stream in listener.incoming() {
    let stream = stream.unwrap();
    pool.execute(|| {
      handle_connection(stream);
    });
  }
}

fn handle_connection(mut stream: TcpStream) {
  let buf_reader = BufReader::new(&mut stream);
  let http_request: Vec<_> = buf_reader
    .lines()
    .map(|result| result.unwrap())
    .take_while(|line| !line.is_empty())
    .collect();
  println!("Request: {:#?}", http_request);

  let status_line = "HTTP/1.1 200 OK";
  let contents = fs::read_to_string("hello.html").unwrap();
  let length = contents.len();
  let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
  stream.write_all(response.as_bytes()).unwrap();
}

/*
  Rust-analyzer
  CodeLLDB
*/