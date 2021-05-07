use std::fs;
use std::io;

use anyhow::Result;

mod editorconfig;
use editorconfig::EditorConfig;

fn main() -> Result<()> {
    let path = ".editorconfig";

    let editor_config = EditorConfig::base();

    let f = fs::File::create(&path)?;
    let mut w = io::BufWriter::new(f);
    write_editor_config(&mut w, &editor_config)?;

    Ok(())
}

fn write_editor_config(w: &mut impl io::Write, editor_config: &EditorConfig) -> Result<()> {
    writeln!(w, "{}", editor_config)?;
    Ok(())
}

#[cfg(test)]
mod genec_test {
    use super::*;

    #[test]
    fn write_editor_config_test() -> Result<()> {
        let editor_config = EditorConfig::base();

        let mut w = Vec::new();
        write_editor_config(&mut w, &editor_config)?;
        assert_eq!(
            w,
            b"\
root = true

[*]
end_of_line = lf
charset = utf-8
trim_trailing_whitespace = true
insert_final_newline = true
"
        );

        Ok(())
    }
}
