use std::*;

pub mod reader;

type Result = result::Result<GraphEvent, String>;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum GraphEvent {
    Node { id: String },
    Edge { id: String, source: String, target: String }
}

impl GraphEvent {
    fn new_node(id: Option<String>) -> Result {
        match id {
            Some(id) => {  Ok(GraphEvent::Node{ id }) }
            None => {
                Err(String::from("node: id missing"))
            }
        }
    }

    fn new_edge(id: Option<String>, source: Option<String>, target: Option<String>) -> result::Result<GraphEvent, String> {
        match ( id, source, target) {
            (Some(id), Some(source), Some(target)) => { Ok(GraphEvent::Edge{id, source, target}) }
            (id, source, target ) => {
                Err(
                    format!("edge: missing field(s) [id: {:?}, source: {:?}, target: {:?}]", id, source, target).to_string()
                )
            }
        }
    }
}

