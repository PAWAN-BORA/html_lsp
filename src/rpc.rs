use core::{panic, str};
use std::u8;
use serde::{Serialize, Deserialize};
use serde_json::{from_slice, to_string, Value};


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct LspRequest {
  pub jsonrpc:String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub id:Option<u32>,
  pub method:String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub params:Option<Value>,

}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Id {
  Interger(u32),
  Null,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct LspResponse {
  pub jsonrpc:String,
  pub id:Id,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub result:Option<Value>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub error:Option<Value>,
}

pub fn encode_message(req:LspResponse)->String{
  let req_json = to_string(&req);
  if req_json.is_ok() {
    let req_str =  req_json.ok().unwrap();
    let response = format!("Content-Length: {}\r\n\r\n{}", req_str.len(), req_str);//r#"Content-Length: \r\n\r\n"#;;
    return  response;
  }else {
    panic!("Error occure in encoding: ${:?}", req_json.err());
  }
}

pub fn decode_message(bytes:&[u8])->Result<LspRequest, String>{
  let seperator = b"\r\n\r\n";
  let (header_bytes, content_bytes) = split_bytes(bytes, seperator)?;
  let header = match str::from_utf8(header_bytes) {
    Ok(h) => h,
    Err(_)=> return Err("Invalid UTF-8 header".into()),
  };
  let content_value = header.lines().find(|line| line.starts_with("Content-Length:")).and_then(|line| line.split(":").nth(1));
  if let Some(value)=content_value{
    let content_length:usize= match value.trim().parse() {
      Ok(val)=>{
        val
      },
      Err(e)=>{
        return Err(format!("Invalid content length: {}", e));
      }
    };
    if content_length > content_bytes.len(){
      return Err("Content-Length exceeds available data".into());
    }
    let content = &content_bytes[..content_length];
    let req_json = from_slice::<LspRequest>(&content);

    if req_json.is_ok() {
      return Ok(req_json.ok().unwrap());
    } else {
      return Err(format!("Invalid content length {:?}", req_json.err()));
    }
  } else {
    return Err("Content-Length not found!".into());
  }
}

pub fn split_bytes<'a>(bytes:&'a [u8], seperator:&[u8])->Result<(&'a [u8], &'a [u8]), String>{
  let mut header:Option<&[u8]> = None;
  let mut content:Option<&[u8]> = None;
  for (i, window) in bytes.windows(seperator.len()).enumerate() {
     if window == seperator {
      header = Some(&bytes[..i]);
      content = Some(&bytes[i+seperator.len()..]);
      break;
    }
  }
  match header {
    Some(value)=>{
      match content {
        Some(content_value)=>{
          Ok((value, content_value))
        },
        None=>{
          return Err("content Not found!".into());
        }
      }
    },
    None=>{
      return Err("Seperator Not found!".into());
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_encode() {
    let res = LspResponse{jsonrpc:"2.0".to_string(), id:Id::Interger(232),result:Some(serde_json::Value::String("text".to_string())), error:None};
    let result = encode_message(res);
    println!("${}", result);
    assert_eq!(result.chars().count(), 64);
  }
  #[test]
  fn test_decode() {
    let req:&str = "Content-Length: 76\r\n\r\n{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"test\",\"params\":{\"name\":\"test\",\"value\":34}}";
    let result = decode_message(req.as_bytes()).unwrap();
    assert_eq!(result.method, "test");
    assert_eq!(result.id.unwrap(), 1);
  }
}
