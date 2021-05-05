pub mod editorconfig {
    pub enum Eol {
        LF,
        CR,
        CRLF,
    }

    pub enum IndentStyle {
        Space,
        Tab,
    }

    pub struct Config {
        pattern: String,
        charset: Option<String>,
        eol: Option<Eol>,
        indent_type: Option<IndentStyle>,
        indent_size: Option<u32>,
        insert_final_newline: Option<bool>,
        trim_trailing_whitespace: Option<bool>,
    }

    pub struct EditorConfig {
        root: bool,
        configs: Vec<Config>,
    }

    impl EditorConfig {
        fn new() -> Self {
            EditorConfig { configs: vec![] }
        }

        fn serialize(self) -> String {
            format!("root = {root}\n", root = "true")
        }
    }

    #[cfg(test)]
    mod editorconfig_test {
        use super::*;

        #[test]
        fn serialize_test() {
            let editorconfig = EditorConfig::new();
            assert_eq!(editorconfig.serialize(), "root = true\n");
        }
    }
}
