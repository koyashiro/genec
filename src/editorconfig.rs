pub mod editorconfig {
    use anyhow::Result;
    use std::io::Write;

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

        fn serialize(&self) -> Result<String> {
            let mut w = Vec::new();
            writeln!(&mut w, "[{}]", &self.pattern)?;

            if let Some(charset) = &self.charset {
                writeln!(&mut w, "charset = {}", &charset)?;
            }

            let serialized = String::from_utf8(w)?;
            Ok(serialized)
        }
    }

    pub struct EditorConfig {
        root: bool,
        configs: Vec<Config>,
    }

    impl EditorConfig {
        fn new() -> Self {
            EditorConfig {
                root: true,
                configs: vec![],
            }
        }

        fn serialize(&self) -> Result<String> {
            let mut w = Vec::new();

            writeln!(
                &mut w,
                "root = {}",
                if self.root { "true" } else { "false" }
            )?;

            for c in &self.configs {
                write!(&mut w, "{}", &c.serialize()?);
            }

            let serialized = String::from_utf8(w)?;
            Ok(serialized)
        }
    }

    #[cfg(test)]
    mod editorconfig_test {
        use super::*;

        #[test]
        fn serialize_test() {
            let mut config = Config::new("*");
            assert_eq!(&config.serialize().unwrap(), "[*]\n");
            config.charset = Some("utf-8".to_string());
            assert_eq!(&config.serialize().unwrap(), "[*]\ncharset = utf-8\n");

            let mut editor_config = EditorConfig::new();
            assert_eq!(&editor_config.serialize().unwrap(), "root = true\n");
            editor_config.configs.push(config);
            assert_eq!(
                &editor_config.serialize().unwrap(),
                "root = true\n[*]\ncharset = utf-8\n"
            );
        }
    }
}
