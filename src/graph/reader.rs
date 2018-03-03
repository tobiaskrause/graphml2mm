use xml::reader::*;
use xml::attribute::OwnedAttribute;
use std::*;
use std::io::*;
use graph::*;

type ReaderResult = result::Result<Vec<GraphEvent>, ReaderError>;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum ReaderError{
    XMLError(String),
    GraphEventError(FieldMissingError)
}

impl fmt::Display for ReaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ReaderError")
    }
}

impl From<FieldMissingError> for ReaderError {
    fn from(err: FieldMissingError) -> ReaderError {
        ReaderError::GraphEventError(err)
    }
}

impl error::Error for ReaderError {
    fn description(&self) -> &str {
        match *self {
            ReaderError::XMLError(ref desc) => { desc }
            ReaderError::GraphEventError(_) => { "encountered a GraphEventError" }
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            ReaderError::GraphEventError(ref err) => { Some(err) }
            _ => None
        }
    }
}

pub fn graphml_reader<R>(read_stream: BufReader<R>) -> ReaderResult
    where R: Read
{
    let parser = EventReader::new(read_stream);
    let mut events: Vec<GraphEvent> = Vec::new();
    let mut result: ReaderResult = Ok(Vec::<GraphEvent>::new());
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, attributes: attrs, .. }) => {
                match name.local_name.as_ref() {
                    "node" => {
                        let node = GraphEvent::new_node(get_value_from_attrs("id", &attrs));
                        match node {
                            Ok(node) => { events.push(node); }
                            Err(err) => {
                                result = Err(ReaderError::from(err));
                                break;
                            }
                        }
                    }
                    "edge" => {
                        let edge = GraphEvent::new_edge(
                            get_value_from_attrs("id", &attrs),
                            get_value_from_attrs("source", &attrs),
                            get_value_from_attrs("target", &attrs),
                            None
                        );
                        match edge {
                            Ok(edge) => { events.push(edge); }
                            Err(err) => {
                                result = Err(ReaderError::from(err));
                                break;
                            }
                        }
                    }
                    _ => {
                        continue;
                    }
                }
            }
            Ok(_) => { continue }
            Err(err) => {
                result = Err(ReaderError::XMLError(err.to_string()));
            }
        }
    }
    result.map(|_| events)
}

fn get_value_from_attrs(val: &str, attrs: &Vec<OwnedAttribute>) -> Option<String> {
    let mut ret = None;
    for attr in attrs {
        if attr.name.local_name == val {
            ret = Some(attr.value.clone())
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use graph::*;
    use graph::GraphEvent::*;
    use graph::reader::ReaderError;
    use std::io::*;

    #[test]
    fn graphml_test() {
        let content = String::from(r#"
    <?xml version="1.0" encoding="UTF-8"?>
        <graphml>
        <graph id="G" edgedefault="undirected">
        <node id="A"/>
        <node id="D"/>
        <edge id="da" source="D" target="A"/>
        </graph>
        </graphml>"#);
        let exp = vec!(
            Node { id: "A".to_string() },
            Node { id: "D".to_string() },
            Edge { id: "da".to_string(), source: "D".to_string(), target: "A".to_string(), data: HashMap::new() }
        );
        assert_result(content, exp);
    }

    #[test]
    fn graphml_no_node_id_test() {
        let content = String::from(r#"
    <?xml version="1.0" encoding="UTF-8"?>
        <graphml>
        <graph id="G" edgedefault="undirected">
        <node/>
        <node id="D"/>
        <edge id="da" source="D" target="A"/>
        </graph>
        </graphml>"#);
        assert_error(content, ReaderError::GraphEventError(FieldMissingError::NodeError(None)));
    }

    #[test]
    fn graphml_no_egde_source_test() {
        let content = String::from(r#"
    <?xml version="1.0" encoding="UTF-8"?>
        <graphml>
        <graph id="G" edgedefault="undirected">
        <edge id="da" target="A"/>
        </graph>
        </graphml>"#);
        assert_error(content,
                     ReaderError::GraphEventError(
                         FieldMissingError::EdgeError(
                             Some("da".to_string()),
                             None,
                             Some("A".to_string()
                             )
                         )
                     )
        );
    }

    #[test]
    fn empty_graphml_test() {
        let content = String::from(r#"
    <?xml version="1.0" encoding="UTF-8"?>
        <graphml>
        <graph id="G" edgedefault="undirected">
        </graph>
            </graphml>"#);
        let exp = vec!();
        assert_result(content, exp);
    }

    #[test]
    fn no_graphml_test() {
        let content = String::from(r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <foo/>"#);
        let exp = vec!();
        assert_result(content, exp);
    }

    #[test]
    fn no_xml_error_test() {
        let content = String::from(r#"
        oo/>"#);
        assert_error(content, ReaderError::XMLError("2:9 Unexpected characters outside the root element: o".to_string()));
    }

    fn assert_result(content: String, exp: Vec<GraphEvent>) {
        let buf_reader = BufReader::new(content.as_bytes());
        let events = reader::graphml_reader(buf_reader).unwrap();

        assert_eq!(&events, &exp);
    }

    fn assert_error(content: String, exp: ReaderError) {
        let buf_reader = BufReader::new(content.as_bytes());
        let err = reader::graphml_reader(buf_reader).unwrap_err();

        assert_eq!(err, exp);
    }
}