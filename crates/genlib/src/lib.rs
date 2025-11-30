pub fn generate(xml: &str) -> String {
    format!(
        "// comment

pub fn dummy_from_schema() -> String {{
    format!(\"len is {}\")
}}
",
        xml.len()
    )
}
