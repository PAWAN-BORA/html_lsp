use serde::{Deserialize, Serialize};

use super::lsp_struct::Range;

#[derive(Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub enum TextDocumentSyncKind {
  None=0,
  Full=1,
  Incremental=2
}
impl Serialize for TextDocumentSyncKind {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where S: serde::Serializer {
    match self {
      Self::None => serializer.serialize_u8(0),
      Self::Full => serializer.serialize_u8(1),
      Self::Incremental => serializer.serialize_u8(2)
    }
  } 
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct Capabilities {
  pub hover_provider:bool,
  pub text_document_sync:TextDocumentSyncKind,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct ServerInfo {
  pub name:String,
  pub version:String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct InitializeResult {
  pub capabilities:Capabilities,
  pub server_info:ServerInfo,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct HoverResult {
  pub contents:String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub range:Option<Range>

}
