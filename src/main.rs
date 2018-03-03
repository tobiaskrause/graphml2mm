extern crate graphml2mm;

use graphml2mm::*;
use std::io::*;

fn main() {

    let content = String::from(r#"
    <?xml version="1.0" encoding="UTF-8"?>
    <graphml>
        <graph id="G">
            <node id="1"/>
            <node id="2"/>
            <node id="3"/>
            <node id="4"/>
            <edge id="1-2" source="1" target="2"/>
            <edge id="2-3" source="2" target="3"/>
            <edge id="3-4" source="3" target="4"/>
            <edge id="4-1" source="4" target="1"/>
        </graph>
    </graphml>"#);

    let buf_reader = BufReader::new(content.as_bytes());
    let events = graph::reader::graphml_reader(buf_reader).unwrap();
    println!("Events:");
    for event in &events {
        println!("{:?}", event);
    }
    println!("Matrix file output:");
    let mut c = Cursor::new(Vec::new());
    graph::writer::mm_writer(&mut c, &events);
    c.seek(SeekFrom::Start(0)).unwrap();
    let mut s = Vec::new();
    c.read_to_end(&mut s).unwrap();
    println!("{}",  String::from_utf8(s).unwrap());

}