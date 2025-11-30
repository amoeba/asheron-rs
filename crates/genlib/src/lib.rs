use quick_xml::{Reader, events::Event};

fn gen_foo(xml: &str) -> String {
    format!(
        "// comment

pub fn dummy_from_schema() -> String {{
    format!(\"len is {}\")
}}
",
        xml.len()
    )
}

pub fn generate(xml: &str) -> String {
    let mut reader = Reader::from_str(xml);
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                let name = e.name().0;
                if name == b"type" {
                    for attr in e.attributes().flatten() {
                        if attr.key.as_ref() == b"name" {
                            let value = attr.unescape_value().unwrap();
                            println!("type name = {value}");
                        }
                    }
                }

                if name == b"enum" {
                    for attr in e.attributes().flatten() {
                        if attr.key.as_ref() == b"name" {
                            let value = attr.unescape_value().unwrap();
                            println!("enum name = {value}");
                        }
                    }
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!("error at position {}: {}", reader.buffer_position(), e),
            _ => {}
        }
        buf.clear();
    }

    gen_foo(xml)
}
