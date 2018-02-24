extern crate graphml2mm;

use graphml2mm::graph::*;
use std::io::*;

fn main() {
    let content = String::from(r#"
    <?xml version="1.0" encoding="UTF-8"?>
    <graphml>
        <graph id="G" edgedefault="undirected">
            <node id="A"/>
            <node id="B"/>
            <node id="C"/>
            <node id="D"/>
            <edge id="ab" source="A" target="B"/>
            <edge id="bc" source="B" target="C"/>
            <edge id="cd" source="C" target="D"/>
            <edge id="da" source="D" target="A"/>
        </graph>
    </graphml>"#);

    let buf_reader = BufReader::new(content.as_bytes());

    let events = reader::graphml_reader(buf_reader).unwrap();

    for event in events {
        println!("{:?}", event);
    }

}