
use graph::*;
use std::io::*;

pub fn  mm_writer<W: Write + Seek>(writer: &mut W, events: &Vec<GraphEvent>) {
    writeln_file_header(writer);
    let header_position = writer.seek(SeekFrom::Current(0)).unwrap();
    writeln_dummy_header(writer, 10);
    let mut entries: usize = 0;
    let mut nodes: usize = 0;

    for event in events {
        match event {
            &GraphEvent::Edge { id: _, ref source, ref target, data: _ } => {
                entries = entries + 1;
                writeln(writer, format!("{} {} {}", source, target, "1.0" ).as_ref());
            }
            &GraphEvent::Node { id: _ } => {
                nodes = nodes + 1;
            }
        }
    }
    writer.seek(SeekFrom::Start(header_position)).unwrap();
    write_header(writer, nodes, nodes, entries);
}

fn writeln_file_header<W: Write + Seek>(writer: &mut W) {
    writer.write(format!("{}\n", "%%MatrixMarket matrix coordinate real general").as_ref()).unwrap();
}


fn write_header<W: Write + Seek>(writer: &mut W, rows: usize, columns: usize, entries: usize ) {
    writer.write(format!("{} {} {}", rows, columns, entries).as_ref()).unwrap();
}


fn writeln_dummy_header<W: Write + Seek>(writer: &mut W, size: usize) {
    let dummy = vec![32; size];
    writer.write(&dummy).unwrap();
    write_line_end(writer);
}

fn writeln<W: Write + Seek>(writer: &mut W, line: &str) {
    writer.write(line.as_ref()).unwrap();
    write_line_end(writer);
}

fn write_line_end<W: Write + Seek>(writer: &mut W) {
    writer.write(format!("\n").as_ref()).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_simple() {
        let mut writer =  Cursor::new(Vec::new());
        let events = vec!(
            GraphEvent::Edge {id: String::from("1"), source: String::from("1"), target: String::from("2"), data: HashMap::new()},
            GraphEvent::Edge {id: String::from("2"), source: String::from("2"), target: String::from("3"), data: HashMap::new()},
            GraphEvent::Node {id: String::from("1")},
            GraphEvent::Node {id: String::from("2")},
            GraphEvent::Node {id: String::from("3")}
        );
        mm_writer(&mut writer, &events);
        writer.seek(SeekFrom::Start(0)).unwrap();
        let mut s = Vec::new();
        writer.read_to_end(&mut s).unwrap();
        assert_eq!(String::from("%%MatrixMarket matrix coordinate real general\n3 3 2     \n1 2 1.0\n2 3 1.0\n"), String::from_utf8(s).unwrap());
    }
}
