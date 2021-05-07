pub mod editorconfig {
    use std::fmt;

    #[allow(dead_code)]
    pub enum Eol {
        LF,
        CR,
        CRLF,
    }

    impl fmt::Display for Eol {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                Eol::LF => write!(f, "lf"),
                Eol::CR => write!(f, "cr"),
                Eol::CRLF => write!(f, "crlf"),
            }
        }
    }

    #[allow(dead_code)]
    pub enum IndentStyle {
        Space,
        Tab,
    }

    impl fmt::Display for IndentStyle {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                IndentStyle::Tab => write!(f, "tab"),
                IndentStyle::Space => write!(f, "space"),
            }
        }
    }

    #[allow(dead_code)]
    pub struct Config {
        pattern: String,
        charset: Option<String>,
        eol: Option<Eol>,
        indent_type: Option<IndentStyle>,
        indent_size: Option<u32>,
        insert_final_newline: Option<bool>,
        trim_trailing_whitespace: Option<bool>,
    }

    #[allow(dead_code)]
    impl Config {
        fn new(pattern: &str) -> Self {
            Config {
                pattern: pattern.to_string(),
                charset: None,
                eol: None,
                indent_type: None,
                indent_size: None,
                insert_final_newline: None,
                trim_trailing_whitespace: None,
            }
        }
    }

    impl fmt::Display for Config {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            writeln!(f, "[{}]", &self.pattern)?;

            if let Some(charset) = &self.charset {
                writeln!(f, "charset = {}", &charset)?;
            }

            if let Some(eol) = &self.eol {
                writeln!(f, "eol = {}", &eol)?;
            }

            if let Some(indent_type) = &self.indent_type {
                writeln!(f, "indent_type = {}", &indent_type)?;
            }

            if let Some(indent_size) = &self.indent_size {
                writeln!(f, "indent_size = {}", &indent_size)?;
            }

            Ok(())
        }
    }

    #[allow(dead_code)]
    pub struct EditorConfig {
        root: bool,
        configs: Vec<Config>,
    }

    #[allow(dead_code)]
    impl EditorConfig {
        fn new() -> Self {
            EditorConfig {
                root: true,
                configs: vec![],
            }
        }
    }

    #[allow(dead_code)]
    impl fmt::Display for EditorConfig {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            writeln!(f, "root = {}", self.root)?;

            for c in &self.configs {
                write!(f, "{}", c)?;
            }

            Ok(())
        }
    }

    #[cfg(test)]
    mod config_test {
        use super::*;

        #[test]
        fn serialize_test() {
            let mut config = Config::new("*");
            assert_eq!(config.to_string(), "[*]\n");

            config.charset = Some("utf-8".to_string());
            assert_eq!(config.to_string(), "[*]\ncharset = utf-8\n");

            config.eol = Some(Eol::LF);
            assert_eq!(config.to_string(), "[*]\ncharset = utf-8\neol = lf\n");
        }
    }

    #[cfg(test)]
    mod editorconfig_test {
        use super::*;

        #[test]
        fn serialize_test() {
            let mut config = Config::new("*");
            assert_eq!(config.to_string(), "[*]\n");

            config.charset = Some("utf-8".to_string());
            assert_eq!(config.to_string(), "[*]\ncharset = utf-8\n");

            let mut editor_config = EditorConfig::new();
            assert_eq!(&editor_config.to_string(), "root = true\n");

            editor_config.configs.push(config);
            assert_eq!(
                &editor_config.to_string(),
                "root = true\n[*]\ncharset = utf-8\n"
            );
        }
    }
}
