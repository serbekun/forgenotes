pub fn tokenize_string(s: &str) -> Vec<String> {
    shell_words::split(s).unwrap()
}