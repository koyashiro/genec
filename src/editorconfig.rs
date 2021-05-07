use std::fmt;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum IndentStyle {
    Space,
    Tab,
    None,
}

impl fmt::Display for IndentStyle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IndentStyle::Tab => write!(f, "tab"),
            IndentStyle::Space => write!(f, "space"),
            IndentStyle::None => Ok(()),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum EndOfLine {
    LF,
    CR,
    CRLF,
    None,
}

impl fmt::Display for EndOfLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EndOfLine::LF => write!(f, "lf"),
            EndOfLine::CR => write!(f, "cr"),
            EndOfLine::CRLF => write!(f, "crlf"),
            EndOfLine::None => Ok(()),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Charset {
    UTF8,
    UTF8BOM,
    UTF16BE,
    UTF16LE,
    Other(String),
    None,
}

impl fmt::Display for Charset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Charset::UTF8 => write!(f, "utf-8"),
            Charset::UTF8BOM => write!(f, "utf-8-bom"),
            Charset::UTF16BE => write!(f, "utf-16-be"),
            Charset::UTF16LE => write!(f, "utf-16-le"),
            Charset::Other(c) => write!(f, "{}", c),
            Charset::None => Ok(()),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub struct Config {
    pub pattern: String,
    pub indent_style: IndentStyle,
    pub indent_size: Option<u32>,
    pub end_of_line: EndOfLine,
    pub charset: Charset,
    pub trim_trailing_whitespace: Option<bool>,
    pub insert_final_newline: Option<bool>,
}

#[allow(dead_code)]
impl Config {
    pub fn new(pattern: &str) -> Self {
        Config {
            pattern: pattern.to_string(),
            indent_style: IndentStyle::None,
            indent_size: None,
            end_of_line: EndOfLine::None,
            charset: Charset::None,
            trim_trailing_whitespace: None,
            insert_final_newline: None,
        }
    }

    pub fn base() -> Self {
        Config {
            pattern: "*".to_string(),
            indent_style: IndentStyle::None,
            indent_size: None,
            end_of_line: EndOfLine::LF,
            charset: Charset::UTF8,
            trim_trailing_whitespace: Some(true),
            insert_final_newline: Some(true),
        }
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", &self.pattern)?;

        match self.indent_style {
            IndentStyle::None => {}
            _ => {
                writeln!(f)?;
                write!(f, "indent_style = {}", &self.indent_style)?;
            }
        }

        if let Some(indent_size) = &self.indent_size {
            writeln!(f)?;
            write!(f, "indent_size = {}", &indent_size)?;
        }

        match self.end_of_line {
            EndOfLine::None => {}
            _ => {
                writeln!(f)?;
                write!(f, "end_of_line = {}", &self.end_of_line)?;
            }
        }

        match self.charset {
            Charset::None => {}
            _ => {
                writeln!(f)?;
                write!(f, "charset = {}", &self.charset)?;
            }
        }

        if let Some(trim_trailing_whitespace) = self.trim_trailing_whitespace {
            writeln!(f)?;
            write!(
                f,
                "trim_trailing_whitespace = {}",
                &trim_trailing_whitespace
            )?;
        }

        if let Some(insert_final_newline) = self.insert_final_newline {
            writeln!(f)?;
            write!(f, "insert_final_newline = {}", &insert_final_newline)?;
        }

        Ok(())
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub struct EditorConfig {
    pub root: bool,
    pub configs: Vec<Config>,
}

#[allow(dead_code)]
impl EditorConfig {
    pub fn new() -> Self {
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
    fn new_test() {
        let pattern = "*";
        let config = Config::new(&pattern);
        assert_eq!(config.pattern, pattern);
        assert_eq!(config.indent_style, IndentStyle::None);
        assert_eq!(config.indent_size, None);
        assert_eq!(config.end_of_line, EndOfLine::None);
        assert_eq!(config.charset, Charset::None);
        assert_eq!(config.trim_trailing_whitespace, None);
        assert_eq!(config.insert_final_newline, None);
    }

    #[test]
    fn base_test() {
        let config = Config::base();
        assert_eq!(config.pattern, "*");
        assert_eq!(config.indent_style, IndentStyle::None);
        assert_eq!(config.indent_size, None);
        assert_eq!(config.end_of_line, EndOfLine::LF);
        assert_eq!(config.charset, Charset::UTF8);
        assert_eq!(config.trim_trailing_whitespace, Some(true));
        assert_eq!(config.insert_final_newline, Some(true));
    }

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
    fn indent_style_test() {
        let mut config = Config::new("*");
        assert_eq!(config.to_string(), "[*]");
        config.indent_style = IndentStyle::Space;
        assert_eq!(config.to_string(), "[*]\nindent_style = space");
        config.indent_style = IndentStyle::Tab;
        assert_eq!(config.to_string(), "[*]\nindent_style = tab");
        config.indent_style = IndentStyle::None;
        assert_eq!(config.to_string(), "[*]");
    }

    #[test]
    fn indent_size_test() {
        let mut config = Config::new("*");
        assert_eq!(config.to_string(), "[*]");
        config.indent_size = Some(2);
        assert_eq!(config.to_string(), "[*]\nindent_size = 2");
        config.indent_size = Some(4);
        assert_eq!(config.to_string(), "[*]\nindent_size = 4");
        config.indent_size = None;
        assert_eq!(config.to_string(), "[*]");
    }

    #[test]
    fn end_of_line_test() {
        let mut config = Config::new("*");
        assert_eq!(config.to_string(), "[*]");
        config.end_of_line = EndOfLine::LF;
        assert_eq!(config.to_string(), "[*]\nend_of_line = lf");
        config.end_of_line = EndOfLine::CR;
        assert_eq!(config.to_string(), "[*]\nend_of_line = cr");
        config.end_of_line = EndOfLine::CRLF;
        assert_eq!(config.to_string(), "[*]\nend_of_line = crlf");
        config.end_of_line = EndOfLine::None;
        assert_eq!(config.to_string(), "[*]");
    }

    #[test]
    fn charset_test() {
        let mut config = Config::new("*");
        assert_eq!(config.to_string(), "[*]");
        config.charset = Charset::UTF8;
        assert_eq!(config.to_string(), "[*]\ncharset = utf-8");
        config.charset = Charset::UTF8BOM;
        assert_eq!(config.to_string(), "[*]\ncharset = utf-8-bom");
        config.charset = Charset::Other("shift_jis".to_string());
        assert_eq!(config.to_string(), "[*]\ncharset = shift_jis");
        config.charset = Charset::None;
        assert_eq!(config.to_string(), "[*]");
    }

    #[test]
    fn trim_trailing_whitespace() {
        let mut config = Config::new("*");
        assert_eq!(config.to_string(), "[*]");
        config.trim_trailing_whitespace = Some(true);
        assert_eq!(config.to_string(), "[*]\ntrim_trailing_whitespace = true");
        config.trim_trailing_whitespace = Some(false);
        assert_eq!(config.to_string(), "[*]\ntrim_trailing_whitespace = false");
        config.trim_trailing_whitespace = None;
        assert_eq!(config.to_string(), "[*]");
    }

    #[test]
    fn insert_final_newline_test() {
        let mut config = Config::new("*");
        assert_eq!(config.to_string(), "[*]");
        config.insert_final_newline = Some(true);
        assert_eq!(config.to_string(), "[*]\ninsert_final_newline = true");
        config.insert_final_newline = Some(false);
        assert_eq!(config.to_string(), "[*]\ninsert_final_newline = false");
        config.insert_final_newline = None;
        assert_eq!(config.to_string(), "[*]");
    }
}

#[cfg(test)]
mod editor_config_test {
    use super::*;

    #[test]
    fn new_test() {
        let editor_config = EditorConfig::new();
        assert!(editor_config.root);
        assert_eq!(editor_config.configs.len(), 0);
    }

    #[test]
    fn editor_config_test() {
        let mut editor_config = EditorConfig::new();
        assert_eq!(&editor_config.to_string(), "root = true");

        let mut config = Config::new("*");
        config.charset = Charset::UTF8;
        config.end_of_line = EndOfLine::LF;
        config.indent_style = IndentStyle::Space;
        config.indent_size = Some(2);
        config.trim_trailing_whitespace = Some(true);
        config.insert_final_newline = Some(true);
        editor_config.configs.push(config);
        assert_eq!(
            &editor_config.to_string(),
            "\
root = true

[*]
indent_style = space
indent_size = 2
end_of_line = lf
charset = utf-8
trim_trailing_whitespace = true
insert_final_newline = true\
"
        );

        let mut config = Config::new("*.rs");
        config.charset = Charset::UTF8;
        config.end_of_line = EndOfLine::LF;
        config.indent_style = IndentStyle::Space;
        config.indent_size = Some(4);
        editor_config.configs.push(config);
        assert_eq!(
            &editor_config.to_string(),
            "\
root = true

[*]
indent_style = space
indent_size = 2
end_of_line = lf
charset = utf-8
trim_trailing_whitespace = true
insert_final_newline = true

[*.rs]
indent_style = space
indent_size = 4
end_of_line = lf
charset = utf-8\
"
        );

        let mut config = Config::new("Makefile");
        config.charset = Charset::UTF8;
        config.end_of_line = EndOfLine::LF;
        config.indent_style = IndentStyle::Tab;
        config.indent_size = Some(4);
        editor_config.configs.push(config);
        assert_eq!(
            &editor_config.to_string(),
            "\
root = true

[*]
indent_style = space
indent_size = 2
end_of_line = lf
charset = utf-8
trim_trailing_whitespace = true
insert_final_newline = true

[*.rs]
indent_style = space
indent_size = 4
end_of_line = lf
charset = utf-8

[Makefile]
indent_style = tab
indent_size = 4
end_of_line = lf
charset = utf-8\
"
        );
    }
}
