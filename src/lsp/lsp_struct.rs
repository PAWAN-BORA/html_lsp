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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct CompletionOptions {
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct CompletionItem {
  pub label:String,
  pub detail:String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub enum DiagnosticSeverity {
  Error=1,
  Warning=2,
  Information=3,
  Hint=4,
}
impl Serialize for DiagnosticSeverity {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer {
    match self {
      Self::Error => serializer.serialize_u8(1),
      Self::Warning=> serializer.serialize_u8(2),
      Self::Information => serializer.serialize_u8(3),
      Self::Hint => serializer.serialize_u8(4),
    }
  }
    
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct Diagnostic {
  pub range:Range,
  pub severity:DiagnosticSeverity,
  pub message:String,
}
