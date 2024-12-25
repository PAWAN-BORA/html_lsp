use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct ClientInfo {
  pub name:String,
  pub version:String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct InitializeParams {
 pub process_id:Option<u32>,
 pub client_info:Option<ClientInfo>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct TextDocumentItem {
  pub uri:String,
  pub language_id:String,
  pub version:u32,
  pub text:String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct DidOpenTextDocumentParams {
  pub text_document:TextDocumentItem,
}
