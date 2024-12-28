use crate::{lsp::lsp_struct::{CompletionItem, Position}};
use super::utils::get_tag_desc;

pub fn get_completion_items(content:&str, position:&Position)->Vec<CompletionItem>{
  let word = get_word(content, position);
  let tag_desc = get_tag_desc();
  let completion_items = tag_desc.iter()
    .filter(|(key, _)| key.starts_with(&word))
    .map(|(key, value)| CompletionItem{
      label:key.to_owned(),
      detail:value.to_string()
    }).collect();

  return completion_items; 
}

fn get_word(content:&str, position:&Position)->String{
  let seperators = [' '];
  if let Some(line) = content.lines().nth(position.line as usize) {
    let mut word = String::new();
    for char in line.chars().rev().skip(line.len() - position.character as usize) {
      if seperators.contains(&char) {
        break;
      }
      word= format!("{}{}", char, word);
    }

    return word;
  } else {

    return String::new(); 
  }
}
