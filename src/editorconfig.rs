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
        indent_style: Option<IndentStyle>,
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
                indent_style: None,
                indent_size: None,
                insert_final_newline: None,
                trim_trailing_whitespace: None,
            }
        }
    }

    impl fmt::Display for Config {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}]", &self.pattern)?;

            if let Some(charset) = &self.charset {
                writeln!(f)?;
                write!(f, "charset = {}", &charset)?;
            }

            if let Some(eol) = &self.eol {
                writeln!(f)?;
                write!(f, "eol = {}", &eol)?;
            }

            if let Some(indent_type) = &self.indent_style {
                writeln!(f)?;
                write!(f, "indent_type = {}", &indent_type)?;
            }

            if let Some(indent_size) = &self.indent_size {
                writeln!(f)?;
                write!(f, "indent_size = {}", &indent_size)?;
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
            write!(f, "root = {}", self.root)?;

            if self.configs.len() > 0 {
                for c in self.configs.iter() {
                    write!(f, "\n\n")?;
                    write!(f, "{}", c)?;
                }
            }

            Ok(())
        }
    }

    #[cfg(test)]
    mod config_test {
        use super::*;

        #[test]
        fn pattern_test() {
            let config = Config::new("*");
            assert_eq!(config.to_string(), "[*]");
            let config = Config::new("*.rs");
            assert_eq!(config.to_string(), "[*.rs]");
            let config = Config::new("Makefile");
            assert_eq!(config.to_string(), "[Makefile]");
        }

        #[test]
        fn charset_test() {
            let mut config = Config::new("*");
            config.charset = Some("utf-8".to_string());
            assert_eq!(config.to_string(), "[*]\ncharset = utf-8");
            config.charset = Some("utf-8-bom".to_string());
            assert_eq!(config.to_string(), "[*]\ncharset = utf-8-bom");
        }

        #[test]
        fn eol_test() {
            let mut config = Config::new("*");
            config.eol = Some(Eol::LF);
            assert_eq!(config.to_string(), "[*]\neol = lf");
            config.eol = Some(Eol::CR);
            assert_eq!(config.to_string(), "[*]\neol = cr");
            config.eol = Some(Eol::CRLF);
            assert_eq!(config.to_string(), "[*]\neol = crlf");
        }
    }

    #[cfg(test)]
    mod editorconfig_test {
        use super::*;

        #[test]
        fn serialize_test() {
            let mut editor_config = EditorConfig::new();
            assert_eq!(&editor_config.to_string(), "root = true");

            let mut config = Config::new("*");
            config.charset = Some("utf-8".to_string());
            config.eol = Some(Eol::LF);
            config.indent_style = Some(IndentStyle::Space);
            config.indent_size = Some(2);
            editor_config.configs.push(config);

            let mut config = Config::new("*.rs");
            config.charset = Some("utf-8".to_string());
            config.eol = Some(Eol::LF);
            config.indent_style = Some(IndentStyle::Space);
            config.indent_size = Some(4);
            editor_config.configs.push(config);

            let mut config = Config::new("Makefile");
            config.charset = Some("utf-8".to_string());
            config.eol = Some(Eol::LF);
            config.indent_style = Some(IndentStyle::Tab);
            config.indent_size = Some(4);
            editor_config.configs.push(config);

            assert_eq!(
                &editor_config.to_string(),
                "\
root = true

[*]
charset = utf-8
eol = lf
indent_type = space
indent_size = 2

[*.rs]
charset = utf-8
eol = lf
indent_type = space
indent_size = 4

[Makefile]
charset = utf-8
eol = lf
indent_type = tab
indent_size = 4\
                "
            );
        }
    }
}
