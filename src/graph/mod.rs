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

#[cfg(test)]
mod tests {
    use graph::*;
    use graph::GraphEvent::*;

    #[test]
    fn node_test() {
        let node = GraphEvent::new_node( Some("A".to_string()));
        assert_eq!(node.unwrap(), Node { id: "A".to_string()});
    }

    #[test]
    fn node_err_test() {
        let node = GraphEvent::new_node( None);
        assert_eq!(node.unwrap_err(), "node: id missing".to_string());
    }

    #[test]
    fn edge_test() {
        let edge = GraphEvent::new_edge( Some("A".to_string()), Some("B".to_string()), Some("C".to_string()));
        assert_eq!(edge.unwrap(), Edge { id: "A".to_string(), source: "B".to_string(), target: "C".to_string()});
    }

    #[test]
    fn edge_err_test() {
        let edge = GraphEvent::new_edge( None, None, None);
        assert_eq!(edge.unwrap_err(), "edge: missing field(s) [id: None, source: None, target: None]");
    }

    #[test]
    fn edge_err_no_id_test() {
        let edge = GraphEvent::new_edge( None, Some("B".to_string()), Some("C".to_string()));
        assert_eq!(edge.unwrap_err(), "edge: missing field(s) [id: None, source: Some(\"B\"), target: Some(\"C\")]");
    }
}
