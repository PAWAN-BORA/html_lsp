use core::{panic, str};
use std::{cell::RefCell, io::{stdin, Read}, u8};

use logger::Logger;
use rpc::{decode_message, split_bytes};
use state::State;
mod rpc;
mod logger;
mod state;
mod lsp;
mod analysis;
// const JSON_STR:&str = "Content-Length: 76\r\n\r\n{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"test\",\"params\":{\"name\":\"test\",\"value\":34}}";
fn main() {
  
  let logger = RefCell::new(Logger::new("/home/pawan/projects/rust/html_lsp/log/log.txt", "[html_lsp]"));
  let mut state = State::new(&logger);
  logger.borrow_mut().info("Main function Started!");
  let stdin = stdin();
  let mut handle = stdin.lock();
  let mut accum_buffer = Vec::new();
  let mut buffer = [0; 4096];
  let seperator = b"\r\n\r\n";
  loop {
    let byte_read = handle.read(&mut buffer).unwrap();
    if byte_read == 0 {
      break;
    }
    accum_buffer.extend_from_slice(&buffer[..byte_read]);

    let res = split_bytes(accum_buffer.as_slice(), seperator);
    let (header_bytes, _) = match res {
      Ok(val)=>val,
      Err(_err)=>continue
    };
    let header = match str::from_utf8(header_bytes) {
      Ok(h) => h,
      Err(_)=> {
        logger.borrow_mut().error("Invalid UTF-8 Header");
        panic!("Invalid UTF-8 Header")
      },
    };

    let content_value = header.lines().find(|line| line.starts_with("Content-Length:")).and_then(|line| line.split(":").nth(1));
    if let Some(value)=content_value{
      let content_length:usize= match value.trim().parse() {
        Ok(val)=>{
          val
        },
        Err(_)=>{
          logger.borrow_mut().error("Invalid content length");
          panic!("Invalid content length")
        }
      };
      let total_length = header_bytes.len() + seperator.len() + content_length; 
      if accum_buffer.len() < total_length{
        continue;
      }
      let message = accum_buffer.drain(..total_length).collect::<Vec<u8>>();
      let lsp_request = decode_message(message.as_slice());
      let msg_str = str::from_utf8(message.as_slice()).unwrap();
      logger.borrow_mut().info(msg_str);
      match lsp_request {

        Ok(value)=>{
          state.handle_message(value);
        },
        Err(e)=>{
          let err_msg = format!("Error on parse: {}", e.as_str());
          logger.borrow_mut().error(&err_msg);
          panic!("Invalid Json");
        }
      }
    }
  }
  logger.borrow_mut().info("End of Main");
}
