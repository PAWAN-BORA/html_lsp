use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct TextDocumentItem {
  pub uri:String,
  pub language_id:String,
  pub version:u32,
  pub text:String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all="camelCase")]
pub struct Position {
  pub line:u32,
  pub character:u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct TextDocumentIdentifier {
  pub uri:String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct Range {
  pub start:Position,
  pub end:Position,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct TextDocumentContentChangeEvent {
  pub text:String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct VersionedTextDocumentIdentifier {
  pub version:u32,
  pub uri:String,

}
