use anyhow::{Context, Result};
use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};

/// Конвертирует exclude-список Duplicati в XML-фильтр TreeSize.
///
/// На вход — текстовый файл, по одному паттерну на строку
/// (пустые строки и строки, начинающиеся с `#`, игнорируются).
#[derive(Parser, Debug)]
#[command(name = "duplicati2treesize", version, about, long_about = None)]
struct Args {
    /// Входной файл (Duplicati exclude). Позиционно или через -i/--input-file.
    #[arg(value_name = "INPUT", index = 1)]
    input_file_pos: Option<PathBuf>,

    #[arg(short = 'i', long = "input-file", value_name = "FILE")]
    input_file_flag: Option<PathBuf>,

    /// Выходной XML для TreeSize. Позиционно или через -o/--output-file.
    #[arg(value_name = "OUTPUT", index = 2)]
    output_file_pos: Option<PathBuf>,

    #[arg(short = 'o', long = "output-file", value_name = "FILE")]
    output_file_flag: Option<PathBuf>,

    /// Не помечать элементы как <Negated>True</Negated>
    #[arg(long)]
    no_negated: bool,

    /// Operator для паттернов, содержащих '*' или '?'
    #[arg(long, default_value_t = 8, value_name = "N")]
    op_wildcard: u32,

    /// Operator для точных путей
    #[arg(long, default_value_t = 10, value_name = "N")]
    op_exact: u32,
}

impl Args {
    fn resolve(&self) -> Result<(PathBuf, PathBuf)> {
        let input = self
            .input_file_flag
            .clone()
            .or_else(|| self.input_file_pos.clone())
            .context("не указан входной файл (позиционно или через -i/--input-file)")?;

        let output = self
            .output_file_flag
            .clone()
            .or_else(|| self.output_file_pos.clone())
            .context("не указан выходной файл (позиционно или через -o/--output-file)")?;

        Ok((input, output))
    }
}

fn xml_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&'  => out.push_str("&amp;"),
            '<'  => out.push_str("&lt;"),
            '>'  => out.push_str("&gt;"),
            '"'  => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            _    => out.push(c),
        }
    }
    out
}

enum ParsedLine {
    Skip,
    Exclude(String),
    IgnoredInclude,
    EmptyPattern,
}

/// Разбирает одну строку filter-файла Duplicati.
/// Формат: `[+-]pattern`, `#`-комментарии, пустые строки.
fn parse_filter_line(raw: &str) -> ParsedLine {
    let t = raw.trim();
    if t.is_empty() || t.starts_with('#') {
        return ParsedLine::Skip;
    }

    // '-' = exclude, '+' = include. Без префикса — считаем exclude (lenient).
    let (is_exclude, rest) = if let Some(r) = t.strip_prefix('-') {
        (true, r.trim())
    } else if let Some(r) = t.strip_prefix('+') {
        (false, r.trim())
    } else {
        (true, t)
    };

    if rest.is_empty() {
        return ParsedLine::EmptyPattern;
    }
    if !is_exclude {
        return ParsedLine::IgnoredInclude;
    }

    // Убираем один хвостовой '\', кроме корней диска вроде "D:\".
    let p = if rest.len() > 3 && rest.ends_with('\\') {
        &rest[..rest.len() - 1]
    } else {
        rest
    };

    ParsedLine::Exclude(p.to_string())
}

fn read_patterns(path: &Path) -> Result<Vec<String>> {
    let file = File::open(path)
        .with_context(|| format!("Не удалось открыть входной файл {}", path.display()))?;
    let reader = BufReader::new(file);

    let mut patterns = Vec::new();
    for (idx, line) in reader.lines().enumerate() {
        let line_no = idx + 1;
        let line = line.with_context(|| format!("Ошибка чтения строки {}", line_no))?;
        match parse_filter_line(&line) {
            ParsedLine::Skip => {}
            ParsedLine::Exclude(p) => patterns.push(p),
            ParsedLine::IgnoredInclude => eprintln!(
                "Предупреждение (строка {line_no}): include-паттерн ('+...') пропускаю — \
                 инструмент конвертирует только exclude: {:?}",
                line.trim()
            ),
            ParsedLine::EmptyPattern => eprintln!(
                "Предупреждение (строка {line_no}): пустой паттерн, пропускаю"
            ),
        }
    }
    Ok(patterns)
}

fn write_xml(
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
        writeln!(w, "\t\t\t\t<fmtid>{{DE632DE0-4253-4BAB-85C4-CA3BE18AA0F6}}</fmtid>")?;
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

fn main() -> Result<()> {
    let args = Args::parse();
    let (input, output) = args.resolve()?;

    let patterns = read_patterns(&input)?;
    if patterns.is_empty() {
        eprintln!("Предупреждение: во входном файле нет валидных паттернов");
    }

    write_xml(
        &output,
        &patterns,
        !args.no_negated,
        args.op_wildcard,
        args.op_exact,
    )?;

    eprintln!("Готово: {} паттернов → {}", patterns.len(), output.display());
    Ok(())
}