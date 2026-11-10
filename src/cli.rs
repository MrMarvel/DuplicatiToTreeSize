use anyhow::{Context, Result};
use clap::Parser;
use std::path::PathBuf;

/// Конвертирует exclude-список Duplicati в XML-фильтр TreeSize.
///
/// На вход — filter-файл Duplicati (`[+-]pattern` на строку; `-` = exclude,
/// `+` = include и пропускается с предупреждением, `#` — комментарий).
#[derive(Parser, Debug)]
#[command(name = "duplicati2treesize", version, about, long_about = None)]
pub struct Args {
    /// Входной файл (Duplicati exclude). Позиционно или через -i/--input-file.
    #[arg(value_name = "INPUT", index = 1)]
    pub input_file_pos: Option<PathBuf>,

    #[arg(short = 'i', long = "input-file", value_name = "FILE")]
    pub input_file_flag: Option<PathBuf>,

    /// Выходной XML для TreeSize. Позиционно или через -o/--output-file.
    #[arg(value_name = "OUTPUT", index = 2)]
    pub output_file_pos: Option<PathBuf>,

    #[arg(short = 'o', long = "output-file", value_name = "FILE")]
    pub output_file_flag: Option<PathBuf>,

    /// Не помечать элементы как <Negated>True</Negated>
    #[arg(long)]
    pub no_negated: bool,

    /// Operator для паттернов, содержащих '*' или '?'
    #[arg(long, default_value_t = 8, value_name = "N")]
    pub op_wildcard: u32,

    /// Operator для точных путей
    #[arg(long, default_value_t = 10, value_name = "N")]
    pub op_exact: u32,
}

impl Args {
    pub fn resolve(&self) -> Result<(PathBuf, PathBuf)> {
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
