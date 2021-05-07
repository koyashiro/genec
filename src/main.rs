use std::fs;
use std::io;

use anyhow::Result;

mod editorconfig;
use editorconfig::{Charset, Config, EditorConfig, EndOfLine, IndentStyle};

fn main() -> Result<()> {
    let path = ".editorconfig";

    let editor_config = create_default_editor_config();

    let f = fs::File::create(&path)?;
    let mut w = io::BufWriter::new(f);
    write_editor_config(&mut w, &editor_config)?;

    Ok(())
}

fn create_default_editor_config() -> EditorConfig {
    let mut editor_config = EditorConfig::new();

    let mut config = Config::new("*");
    config.indent_style = IndentStyle::Space;
    config.indent_size = Some(2);
    config.end_of_line = EndOfLine::LF;
    config.charset = Charset::UTF8;
    config.trim_trailing_whitespace = Some(true);
    config.insert_final_newline = Some(true);
    editor_config.configs.push(config);

    editor_config
}

fn write_editor_config(w: &mut impl io::Write, editor_config: &EditorConfig) -> Result<()> {
    writeln!(w, "{}", editor_config)?;
    Ok(())
}

#[cfg(test)]
mod genec_test {
    use super::*;

    #[test]
    fn create_default_editor_config_test() {
        let editor_config = create_default_editor_config();
        assert_eq!(editor_config.root, true);
        assert_eq!(editor_config.configs.len(), 1);

        let config = &editor_config.configs[0];
        assert_eq!(config.pattern, "*");
        assert_eq!(config.indent_style, IndentStyle::Space);
        assert_eq!(config.indent_size, Some(2));
        assert_eq!(config.end_of_line, EndOfLine::LF);
        assert_eq!(config.charset, Charset::UTF8);
        assert_eq!(config.trim_trailing_whitespace, Some(true));
        assert_eq!(config.insert_final_newline, Some(true));
    }

    #[test]
    fn write_editor_config_test() -> Result<()> {
        let editor_config = create_default_editor_config();

        let mut w = Vec::new();
        write_editor_config(&mut w, &editor_config)?;
        assert_eq!(
            w,
            b"\
root = true

[*]
indent_style = space
indent_size = 2
end_of_line = lf
charset = utf-8
trim_trailing_whitespace = true
insert_final_newline = true
"
        );

        Ok(())
    }
}
