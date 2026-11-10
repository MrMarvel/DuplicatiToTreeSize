use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub enum ParsedLine {
    Skip,
    Exclude(String),
    IgnoredInclude,
    EmptyPattern,
}

/// Разбирает одну строку filter-файла Duplicati.
pub fn parse_filter_line(raw: &str) -> ParsedLine {
    let t = raw.trim();
    if t.is_empty() || t.starts_with('#') {
        return ParsedLine::Skip;
    }

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

    let p = if rest.len() > 3 && rest.ends_with('\\') {
        &rest[..rest.len() - 1]
    } else {
        rest
    };

    ParsedLine::Exclude(p.to_string())
}

pub fn read_patterns(path: &Path) -> Result<Vec<String>> {
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
                "Предупреждение (строка {line_no}): include-паттерн пропущен: {:?}",
                line.trim()
            ),
            ParsedLine::EmptyPattern => {
                eprintln!("Предупреждение (строка {line_no}): пустой паттерн, пропускаю")
            }
        }
    }
    Ok(patterns)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn excl(raw: &str) -> Option<String> {
        match parse_filter_line(raw) {
            ParsedLine::Exclude(s) => Some(s),
            _ => None,
        }
    }

    #[test]
    fn strips_exclude_marker() {
        assert_eq!(
            excl("-D:\\TEMP\\Microsoft\\").as_deref(),
            Some("D:\\TEMP\\Microsoft")
        );
    }

    #[test]
    fn keeps_drive_root() {
        assert_eq!(excl("-D:\\").as_deref(), Some("D:\\"));
    }

    #[test]
    fn keeps_wildcards() {
        assert_eq!(excl("-D:\\TEMP\\*.xml").as_deref(), Some("D:\\TEMP\\*.xml"));
    }

    #[test]
    fn skips_comments_and_blanks() {
        assert!(excl("").is_none());
        assert!(excl("   ").is_none());
        assert!(excl("# comment").is_none());
    }

    #[test]
    fn include_is_not_exclude() {
        assert!(excl("+D:\\keep").is_none());
    }
}
