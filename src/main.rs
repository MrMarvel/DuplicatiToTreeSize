mod cli;
mod parser;
mod tree_size;

use anyhow::Result;
use clap::Parser;
fn main() {
    if let Err(e) = run() {
        eprintln!("Ошибка: {e:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let args = cli::Args::parse();
    let (input, output) = args.resolve()?;

    let patterns = parser::read_patterns(&input)?;
    if patterns.is_empty() {
        eprintln!("Предупреждение: во входном файле нет валидных паттернов");
    }

    tree_size::write_xml(
        &output,
        &patterns,
        !args.no_negated,
        args.op_wildcard,
        args.op_exact,
    )?;

    eprintln!(
        "Готово: {} паттернов → {}",
        patterns.len(),
        output.display()
    );
    Ok(())
}
