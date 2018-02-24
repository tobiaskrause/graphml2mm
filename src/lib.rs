extern crate xml;

pub mod graph {
#[derive(Debug)]
    pub enum GraphEvent {
        Node { id: String },
        Edge { id: String, source: String, target: String }
    }

    pub mod reader {
        use xml::reader::*;
        use xml::attribute::OwnedAttribute;
        use std::*;
        use std::io::*;
        use graph::*;

        pub fn graphml_reader<R>(read_stream: BufReader<R>) -> io::Result<Vec<GraphEvent>>
            where R: Read
        {
            let parser = EventReader::new(read_stream);
            let mut events: Vec<GraphEvent> = Vec::new();
            for e in parser {
                match e {
                    Ok(XmlEvent::StartElement { name, attributes: attrs, .. }) => {
                        let id_opt = get_value_from_attrs("id", &attrs);
                        if id_opt.is_none() {
                            continue;
                        }
                        let id = id_opt.unwrap();
                        match name.local_name.as_ref() {
                            "node" => {
                                events.push(GraphEvent::Node{ id});
                            }
                            "edge" => {
                                let source = get_value_from_attrs("source", &attrs).unwrap();
                                let target = get_value_from_attrs("target", &attrs).unwrap();
                                events.push(GraphEvent::Edge {id, source, target});
                            }
                            _ => {
                                continue;
                            }
                        }
                    }
                    Ok(_) => {}
                    Err(e) => {
                        println!("Error: {}", e);
                        break;
                    }
                }
            }
            Ok(events)
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
    }
}