/// Приводит паттерн Duplicati к виду, понятному TreeSize.
pub fn to_tree_size_pattern(duplicati_pattern: &str) -> String {
    let mut p = duplicati_pattern
        .strip_prefix('*')
        .unwrap_or(duplicati_pattern);

    if p.len() > 3 && p.ends_with('\\') && !(p.ends_with("\\*") || p.ends_with("\\*\\*")) {
        p = &p[..p.len() - 1];
        return format!("{p}\\*");
    }

    p.to_string()
}

/// Пакетная конверсия — удобно для main.
pub fn to_tree_size_patterns(raw: &[String]) -> Vec<String> {
    raw.iter().map(|p| to_tree_size_pattern(p)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_leading_star() {
        assert_eq!(
            to_tree_size_pattern("*D:\\Users\\Sergey\\AppData\\Local\\JetBrains\\PyCharm*\\"),
            "D:\\Users\\Sergey\\AppData\\Local\\JetBrains\\PyCharm*\\*"
        );
    }

    #[test]
    fn folder_backslash_becomes_star() {
        assert_eq!(
            to_tree_size_pattern("D:\\TEMP\\Microsoft\\"),
            "D:\\TEMP\\Microsoft\\*"
        );
    }

    #[test]
    fn file_pattern_unchanged() {
        assert_eq!(
            to_tree_size_pattern("D:\\TEMP\\file.txt"),
            "D:\\TEMP\\file.txt"
        );
    }

    #[test]
    fn drive_root_unchanged() {
        assert_eq!(to_tree_size_pattern("D:\\"), "D:\\");
    }

    #[test]
    fn does_not_duplicate_star() {
        assert_eq!(to_tree_size_pattern("D:\\folder\\*"), "D:\\folder\\*");
        assert_eq!(to_tree_size_pattern("D:\\folder\\*\\*"), "D:\\folder\\*\\*");
    }
}
