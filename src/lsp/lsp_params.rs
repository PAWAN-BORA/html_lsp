use serde::{Deserialize, Serialize};

use super::lsp_struct::{Position, TextDocumentContentChangeEvent, TextDocumentIdentifier, TextDocumentItem, VersionedTextDocumentIdentifier};


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
pub struct DidOpenTextDocumentParams {
  pub text_document:TextDocumentItem,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct DidChangeTextDocumentParams {
  pub text_document:VersionedTextDocumentIdentifier,
  pub content_changes:Vec<TextDocumentContentChangeEvent>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct TextDocumentPositionParams {
  pub text_document:TextDocumentIdentifier,
  pub position:Position,

}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct HoverParams {
  #[serde(flatten)]
  pub text_doucment_position_params:TextDocumentPositionParams,
}
