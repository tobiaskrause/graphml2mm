use xml::reader::*;
use xml::attribute::OwnedAttribute;
use std::*;
use std::io::*;
use graph::*;

type Result = result::Result<Vec<GraphEvent>, String>;

pub fn graphml_reader<R>(read_stream: BufReader<R>) -> Result
    where R: Read
{
    let parser = EventReader::new(read_stream);
    let mut events: Vec<GraphEvent> = Vec::new();
    let mut result: Result = Ok(Vec::<GraphEvent>::new());
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, attributes: attrs, .. }) => {
                match name.local_name.as_ref() {
                    "node" => {
                        let node = GraphEvent::new_node(get_value_from_attrs("id", &attrs));
                        match node {
                            Ok(node) => { events.push(node); }
                            Err(err) => {
                                result = Err(err);
                                break;
                            }
                        }
                    }
                    "edge" => {
                        let edge = GraphEvent::new_edge(
                            get_value_from_attrs("id", &attrs),
                            get_value_from_attrs("source", &attrs),
                            get_value_from_attrs("target", &attrs),
                        );
                        match edge {
                            Ok(edge) => { events.push(edge); }
                            Err(err) => {
                                result = Err(err);
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
            Err(e) => {
                result = Err(e.to_string());
                //println!("Error: {}", e);
                //break
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
            Edge { id: "da".to_string(), source: "D".to_string(), target: "A".to_string() }
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
        assert_error(content, "node: id missing".to_string());
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
        assert_error(content, "edge: missing field(s) [id: Some(\"da\"), source: None, target: Some(\"A\")]".to_string());
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

    fn assert_result(content: String, exp: Vec<GraphEvent>) {
        let buf_reader = BufReader::new(content.as_bytes());
        let events = reader::graphml_reader(buf_reader).unwrap();

        assert_eq!(&events, &exp);
    }

    fn assert_error(content: String, exp: String) {
        let buf_reader = BufReader::new(content.as_bytes());
        let err = reader::graphml_reader(buf_reader).unwrap_err();

        assert_eq!(err, exp);
    }
}