use std::{fs::{File, OpenOptions}, io::Write};

use chrono::Local;


#[derive(Debug)]
pub struct Logger {
  pub prefix:String,
  // pub file:Mutex<std::fs::File>
  pub file:File
}

impl Logger {
  pub fn new(file_path:&str, prefix:&str)->Self{
    let file = OpenOptions::new().create(true).write(true).truncate(true).read(true).open(file_path).expect("unable to open file");
    // let file = File::create(file_path).expect("unable to open file");
    Logger {
      file:file,
      prefix:prefix.to_string()
    }
  }
  #[track_caller]
  fn write_to_file(&mut self, level:String, msg:String) {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let location = std::panic::Location::caller();
    let log_message = format!("{} {} {},[{}:{}:{}] {}\n", level, self.prefix, timestamp, location.file(), location.line(), location.column(), msg);
    self.file.write_all(log_message.as_bytes()).expect("can not write into file");
  }
  #[track_caller]
  pub fn info(&mut self, msg:&str){
    self.write_to_file("info".to_string(), msg.to_string());
  }
  #[track_caller]
  pub fn error(&mut self, msg:&str){
    self.write_to_file("error".to_string(), msg.to_string());
  }
  #[track_caller]
  pub fn warn(&mut self, msg:&str){
    self.write_to_file("warn".to_string(), msg.to_string());
  }
}
