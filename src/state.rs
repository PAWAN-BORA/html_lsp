use std::{borrow::BorrowMut, cell::RefCell, collections::HashMap, io::{stdout, Stdout, Write} };

use serde_json::{from_value, to_value, Value};

use crate::{logger::Logger, lsp::{lsp_params::{DidOpenTextDocumentParams, InitializeParams}, lsp_results::{Capabilities, InitializeResult, ServerInfo, TextDocumentSyncKind}}, rpc::{encode_message, Id, LspRequest, LspResponse}};



pub struct State<'a> {
  logger:&'a RefCell<Logger>,
  writer:Stdout,
  documents:HashMap<String, String>
}

impl <'a>State<'a> {
  pub fn new(logger:&'a RefCell<Logger>)->Self{
    State { 
      logger: logger,
      documents:HashMap::new(),
      writer:stdout()
    }
  }

  pub fn handle_message(&mut self, lsp_request:LspRequest){

    self.logger.borrow_mut().info("inside the handle message");
    match lsp_request.method.as_str() {
      "initialize"=>{
        let initialize_params:InitializeParams = from_value(lsp_request.params.unwrap()).unwrap();
        let id = lsp_request.id.unwrap();
        let client_info = initialize_params.client_info;
        match client_info{
          Some(value)=>{
            self.logger.borrow_mut().info(format!("name:{} varsion:{}", value.name, value.version).as_str());
            let lsp_response =  self.initialize_response(id);
            self.write_response(lsp_response);

          }
          None => {
            self.logger.borrow_mut().warn("client info not found!");
          }
        }
        self.logger.borrow_mut().info("initialize method");
      }
      "initialized"=>{
        self.logger.borrow_mut().info("LSP connected!");
      }
      "textDocument/didOpen"=>{
        let did_open_text_document_params:DidOpenTextDocumentParams = from_value(lsp_request.params.unwrap()).unwrap();
        self.add_or_update_doucment(did_open_text_document_params.text_document.uri, did_open_text_document_params.text_document.text);
        self.logger.borrow_mut().info("Document did opened!");
      }
      "textDocument/didChange"=>{
        self.logger.borrow_mut().info("Document did change!");
        // self.add_or_update_doucment(did_open_text_document_params);
      }
      "textDocument/hover"=>{
        self.logger.borrow_mut().info("Document hovered!");
        // self.add_or_update_doucment(did_open_text_document_params);
      }
      _ =>{
        let msg = format!("Method not found: {}", lsp_request.method);
        self.logger.borrow_mut().info(&msg);
      }

    }
  }
  fn initialize_response(&self, id:u32)->LspResponse{
    let initialize_result:InitializeResult = InitializeResult {
      capabilities:Capabilities{
        hover_provider:true,
        text_document_sync:TextDocumentSyncKind::Full,
      },
      server_info:ServerInfo {
        name:"html_lsp".to_string(),
        version:"0.0.1".to_string()
      }
    };
    let initialize_value = to_value(initialize_result).unwrap();
    let lsp_response = LspResponse {
      jsonrpc:"2.0".to_string(),
      id:Id::Interger(id),
      result:Some(initialize_value),
      error:None,
    };

    return lsp_response
  }
  fn add_or_update_doucment(&mut self, uri:String, text:String){
    self.documents.insert(uri, text);
  }
  fn write_response(&mut self, lsp_response:LspResponse){
    let msg = encode_message(lsp_response);
    self.writer.write(msg.as_bytes()).expect("Failed to write");
    self.writer.flush().expect("Failed to flush");
    self.logger.borrow_mut().info(msg.as_str());
  }
    
}
