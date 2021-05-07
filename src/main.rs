use std::fs;
use std::io;

use anyhow::Result;

mod editorconfig;

fn main() -> Result<()> {
    let path = ".editorconfig";

    let mut config = editorconfig::Config::new("*");
    config.indent_style = editorconfig::IndentStyle::Space;
    config.indent_size = Some(2);
    config.end_of_line = editorconfig::EndOfLine::LF;
    config.charset = editorconfig::Charset::UTF8;
    config.trim_trailing_whitespace = Some(true);
    config.insert_final_newline = Some(true);
    let mut editor_config = editorconfig::EditorConfig::new();
    editor_config.configs.push(config);

    let f = fs::File::create(&path)?;
    let mut w = io::BufWriter::new(f);
    write_editor_config(&mut w, &editor_config)?;

    Ok(())
}

fn write_editor_config(
    w: &mut impl io::Write,
    editor_config: &editorconfig::EditorConfig,
) -> Result<()> {
    writeln!(w, "{}", editor_config)?;
    Ok(())
}

#[cfg(test)]
mod genec_test {
    use super::*;

    #[test]
    fn write_editor_config_test() -> Result<()> {
        let mut config = editorconfig::Config::new("*");
        config.indent_style = editorconfig::IndentStyle::Space;
        config.indent_size = Some(2);
        config.end_of_line = editorconfig::EndOfLine::LF;
        config.charset = editorconfig::Charset::UTF8;
        config.trim_trailing_whitespace = Some(true);
        config.insert_final_newline = Some(true);

        let mut editor_config = editorconfig::EditorConfig::new();
        editor_config.configs.push(config);

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
