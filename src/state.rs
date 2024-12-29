use std::{cell::RefCell, collections::HashMap, io::{stdout, Stdout, Write} };

use serde::Serialize;
use serde_json::{from_value, to_value};

use crate::{analysis::{completion_analysis::get_completion_items, diagnostic::{get_diagnostics}, hover_analysis::get_hover_data}, logger::Logger, lsp::{lsp_params::{CompletionParams, DidChangeTextDocumentParams, DidOpenTextDocumentParams, HoverParams, InitializeParams, PublishDiagnosticsParams}, lsp_results::{Capabilities, HoverResult, InitializeResult, ServerInfo, TextDocumentSyncKind}, lsp_struct::{CompletionItem, CompletionOptions, Position}}, rpc::{encode_message, Id, LspRequest, LspResponse, NotificationMessage}};



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
        let uri = did_open_text_document_params.text_document.uri;
        self.add_or_update_doucment(uri.clone(), did_open_text_document_params.text_document.text);
        // self.logger.borrow_mut().info("Document did opened!");
      }
      "textDocument/didChange"=>{
        let did_change_params:DidChangeTextDocumentParams = from_value(lsp_request.params.unwrap()).unwrap();
        for text_doc in did_change_params.content_changes {
          let uri = did_change_params.text_document.uri.clone();
          self.add_or_update_doucment(uri, text_doc.text);
        }
      }
      "textDocument/completion"=>{
        let completion_params:CompletionParams= from_value(lsp_request.params.unwrap()).unwrap();
        let id = lsp_request.id.unwrap();
        let lsp_response = self.completion_response(id, completion_params.text_document_position_params.text_document.uri, completion_params.text_document_position_params.position);
        self.write_response(lsp_response);
        self.logger.borrow_mut().info("ask for complition");
      }
      "textDocument/hover"=>{
        self.logger.borrow_mut().info("Document hovered!");
        let hover_params:HoverParams= from_value(lsp_request.params.unwrap()).unwrap();
        let id = lsp_request.id.unwrap();
        let lsp_response = self.hover_response(id, hover_params.text_document_position_params.text_document.uri, hover_params.text_document_position_params.position);
        self.write_response(lsp_response);
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
        completion_provider:CompletionOptions {}

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
    self.documents.insert(uri.clone(), text.clone());
    let diagnostics = get_diagnostics(text);
    let params = PublishDiagnosticsParams {
      uri:uri,
      diagnostics:diagnostics,

    };
    let notification = NotificationMessage {
      jsonrpc:"2.0".to_string(),
      method:"textDocument/publishDiagnostics".to_string(),
      params:to_value(params).unwrap(),
    };
    self.write_response(notification);
  }
  fn completion_response(&self, id:u32, uri:String, position:Position)->LspResponse {

    // let logger = 
    let empty_val = "".to_string();
    let content = self.documents.get(&uri).unwrap_or(&empty_val);
    let completion_result:Vec<CompletionItem> = get_completion_items(content, &position); 
    let lsp_response = LspResponse {
      jsonrpc:"2.0".to_string(),
      id:Id::Interger(id),
      result:Some(to_value(completion_result).unwrap()),
      error:None,
    };
    return lsp_response;
  }
  fn hover_response(&self, id:u32, uri:String, position:Position)->LspResponse{
    let empty_val = "".to_string();
    let content = self.documents.get(&uri).unwrap_or(&empty_val);
    let hover_data = get_hover_data(content, &position);
    let hover_result = HoverResult {
      contents:hover_data,
      range:None
    };

    let lsp_response = LspResponse {
      jsonrpc:"2.0".to_string(),
      id:Id::Interger(id),
      result:Some(to_value(hover_result).unwrap()),
      error:None,
    };
    return lsp_response;

  }
  fn write_response<T:Serialize>(&mut self, lsp_response:T){
    let msg = encode_message(lsp_response);
    self.writer.write_all(msg.as_bytes()).expect("Failed to write");
    self.writer.flush().expect("Failed to flush");
  }
    
}
