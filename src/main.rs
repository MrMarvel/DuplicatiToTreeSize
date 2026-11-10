mod cli;
mod convert;
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

    let patterns_tree_size = convert::to_tree_size_patterns(&patterns);
    tree_size::write_xml(&output, &patterns_tree_size, args.invert_negating)?;

    eprintln!(
        "Готово: {} паттернов → {}",
        patterns_tree_size.len(),
        output.display()
    );
    Ok(())
}
