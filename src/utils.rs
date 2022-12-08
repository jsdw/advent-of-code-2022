macro_rules! regex {
    ($pat:literal) => {{
        regex::Regex::new($pat).expect("valid pattern expected")
    }}
}