use std::*;
use std::collections::*;

pub mod reader;
pub mod writer;

type GraphEventResult = result::Result<GraphEvent, FieldMissingError>;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum FieldMissingError {
    NodeError(Option<String>),
    EdgeError(Option<String>, Option<String>, Option<String>)
}

impl fmt::Display for FieldMissingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FieldMissingError::NodeError(_) => { write!(f, "node: missing id field") }
            FieldMissingError::EdgeError(ref id, ref source, ref target) => {
                write!(f, "edge: missing field(s) [id: {:?}, source: {:?}, target: {:?}]", id, source, target)
            }
        }
    }
}

impl error::Error for FieldMissingError {
    fn description(&self) -> &str {
        "Missing field while parsing"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum GraphEvent {
    Node { id: String },
    Edge { id: String, source: String, target: String, attrs: HashMap<String,String> }
}

impl GraphEvent {
    fn new_node(id: Option<String>) -> GraphEventResult {
        match id {
            Some(id) => { Ok(GraphEvent::Node{id}) }
            None => {
                Err(FieldMissingError::NodeError(None))
            }
        }
    }

    fn new_edge(id: Option<String>, source: Option<String>, target: Option<String>, attrs: Option<HashMap<String,String>>) -> GraphEventResult {
        match ( id, source, target, attrs) {
            (Some(id), Some(source), Some(target), None) => { Ok(GraphEvent::Edge{id, source, target, attrs: HashMap::new()}) }
            (Some(id), Some(source), Some(target), Some(attrs)) => { Ok(GraphEvent::Edge{id, source, target, attrs}) }
            (id, source, target, _ ) => { Err(FieldMissingError::EdgeError(id, source, target)) }
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
        assert_eq!(node.unwrap_err(), FieldMissingError::NodeError(None));
    }

    #[test]
    fn edge_test() {
        let edge = GraphEvent::new_edge( Some("A".to_string()), Some("B".to_string()), Some("C".to_string()), None);
        assert_eq!(edge.unwrap(), Edge { id: "A".to_string(), source: "B".to_string(), target: "C".to_string(), attrs: HashMap::new()});
    }

    #[test]
    fn edge_err_test() {
        let edge = GraphEvent::new_edge( None, None, None, None);
        assert_eq!(edge.unwrap_err(),FieldMissingError::EdgeError(None, None , None, ));
    }

    #[test]
    fn edge_err_no_id_test() {
        let edge = GraphEvent::new_edge( None, Some("B".to_string()), Some("C".to_string()), None);
        assert_eq!(edge.unwrap_err(), FieldMissingError::EdgeError(None,Some("B".to_string()),Some("C".to_string())));
    }
}
