use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

fn xml_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            _ => out.push(c),
        }
    }
    out
}

pub fn write_xml(
    path: &Path,
    patterns: &[String],
    negated: bool,
    op_wildcard: u32,
    op_exact: u32,
) -> Result<()> {
    let file = File::create(path)
        .with_context(|| format!("Не удалось создать выходной файл {}", path.display()))?;
    let mut w = BufWriter::new(file);

    let negated_str = if negated { "True" } else { "False" };

    writeln!(w, r#"<TContainerExpression Type="TContainerExpression">"#)?;
    for pattern in patterns {
        let is_wildcard = pattern.contains('*') || pattern.contains('?');
        let op = if is_wildcard { op_wildcard } else { op_exact };

        writeln!(w, "\t<Item Type=\"TStringExpression\">")?;
        writeln!(w, "\t\t<Value>{}</Value>", xml_escape(pattern))?;
        writeln!(w, "\t\t<Operator>{op}</Operator>")?;
        writeln!(w, "\t\t<WholeWordOnly>False</WholeWordOnly>")?;
        writeln!(w, "\t\t<CaseSensitive>False</CaseSensitive>")?;
        writeln!(w, "\t\t<MetaData>")?;
        writeln!(w, "\t\t\t<Column>")?;
        writeln!(
            w,
            "\t\t\t\t<fmtid>{{DE632DE0-4253-4BAB-85C4-CA3BE18AA0F6}}</fmtid>"
        )?;
        writeln!(w, "\t\t\t\t<pid>0</pid>")?;
        writeln!(w, "\t\t\t</Column>")?;
        writeln!(w, "\t\t</MetaData>")?;
        writeln!(w, "\t\t<Negated>{negated_str}</Negated>")?;
        writeln!(w, "\t\t<Active>True</Active>")?;
        writeln!(w, "\t</Item>")?;
    }
    writeln!(w, "</TContainerExpression>")?;
    w.flush()?;

    Ok(())
}
