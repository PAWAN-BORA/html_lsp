use crate::{lsp::lsp_struct::{Diagnostic, DiagnosticSeverity, Position, Range}};

pub fn get_diagnostics(content:String)->Vec<Diagnostic> {
  let start_char = '<';
  let close_char = '>';
  let mut diagnostics:Vec<Diagnostic> = Vec::new();
  let mut obj = (false, Position {
    line:0, character:0
  }, "".to_string());
  for (i, line) in content.lines().enumerate() {
    let i = i as u32;
    let mut peakable = line.chars().enumerate().peekable();
    while let Some((j, char)) = peakable.next() {
      let j = j as u32;
      if char == start_char {
        if obj.0 {
          diagnostics.push(Diagnostic {
            range:Range {
              start:obj.1,
              end:Position {line:i, character:j},
            },
            severity:DiagnosticSeverity::Error,
            message:"Angle bracket is not closed".to_string()
          });
        } else {
          obj.0 = true;
          obj.1.line = i;
          obj.1.character = j+1;
          if let Some((_, next_char)) = peakable.peek() {
            if *next_char == ' ' {
              diagnostics.push(Diagnostic {
                range:Range {
                  start:Position {line:i, character:j},
                  end:Position {line:i, character:j+1},
                },
                severity:DiagnosticSeverity::Error,
                message:"Extra space".to_string()
              });
            }
          }
        }

      } else if char == close_char {
        if obj.0 {
          obj.0 = false;
          obj.2 = "".to_string();
        } else {
          diagnostics.push(Diagnostic {
            range:Range {
              start:Position {line:i, character:j},
              end:Position {line:i, character:j},
            },
            severity:DiagnosticSeverity::Error,
            message:"Angle bracket is not started".to_string()
          });
        }
      } 
    }
    
  }
  return diagnostics;
}

