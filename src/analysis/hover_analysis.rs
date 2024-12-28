use crate::lsp::lsp_struct::Position;

use super::utils::get_tag_desc;

pub fn get_hover_data(content:&str, position:&Position)->String {

  let desc_list = get_tag_desc();
  let word = get_word(content, position);
  let empty_val = String::from("");
  let desc = desc_list.get(&word).unwrap_or(&empty_val);
  return desc.to_string();
}

fn get_word(content:&str, position:&Position)->String{
  let seperators = ['<', '>', '/'];
  let mut is_starting = false;
  if let Some(line) = content.lines().nth(position.line as usize) {
    let mut word = String::from(""); 
    for (index, char) in line.chars().skip(position.character as usize).enumerate() {
      if index==0 && char=='<' {
        is_starting = true;
        continue;
      } else if char=='/' {
        continue;
      } else if seperators.contains(&char) {
        break;
      }
      if char==' ' {
        break;
      }
      word.push(char);
    }
    if is_starting {
      return word;
    }
    for char in line.chars().rev().skip(line.len() - position.character as usize) {
      if seperators.contains(&char) {
        break;
      }
      if char==' ' {
        return "".to_string();
      }
      word= format!("{}{}", char, word);
    }

    return word;

  } else {
    return String::from("");
  }
}
