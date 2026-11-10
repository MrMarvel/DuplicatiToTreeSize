use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

/// TreeSize operator «путь не соответствует шаблону» — используется для всех паттернов.
const TREE_SIZE_TEMPLATE_OPERATOR: u32 = 10;

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

pub fn write_xml(path: &Path, patterns: &[String], negated: bool) -> Result<()> {
    let file = File::create(path)
        .with_context(|| format!("Не удалось создать выходной файл {}", path.display()))?;
    let mut w = BufWriter::new(file);

    let negated_str = if negated { "True" } else { "False" };

    writeln!(w, r#"<TContainerExpression Type="TContainerExpression">"#)?;
    writeln!(w, "\t<Item Type=\"TExpressionAnd\">")?;
    for pattern in patterns {
        let op = TREE_SIZE_TEMPLATE_OPERATOR;

        writeln!(w, "\t\t<Item Type=\"TStringExpression\">")?;
        writeln!(w, "\t\t\t\t<Value>{}</Value>", xml_escape(pattern))?;
        writeln!(w, "\t\t\t\t<Operator>{op}</Operator>")?;
        writeln!(w, "\t\t\t\t<WholeWordOnly>False</WholeWordOnly>")?;
        writeln!(w, "\t\t\t\t<CaseSensitive>False</CaseSensitive>")?;
        writeln!(w, "\t\t\t\t<MetaData>")?;
        writeln!(w, "\t\t\t\t\t\t<Column>")?;
        writeln!(
            w,
            "\t\t\t\t\t\t\t\t<fmtid>{{DE632DE0-4253-4BAB-85C4-CA3BE18AA0F6}}</fmtid>"
        )?;
        writeln!(w, "\t\t\t\t\t\t\t\t<pid>1</pid>")?;
        writeln!(w, "\t\t\t\t\t\t</Column>")?;
        writeln!(w, "\t\t\t\t</MetaData>")?;
        writeln!(w, "\t\t\t\t<Negated>{negated_str}</Negated>")?;
        writeln!(w, "\t\t\t\t<Active>True</Active>")?;
        writeln!(w, "\t\t</Item>")?;
    }
    writeln!(w, "\t</Item>")?;
    writeln!(w, "</TContainerExpression>")?;
    w.flush()?;

    Ok(())
}
