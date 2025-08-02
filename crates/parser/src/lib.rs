use regex::Regex;

/// Parse block references like ((block-id))
pub fn extract_block_refs(text: &str) -> Vec<String> {
    let re = Regex::new(r"\(\(([0-9a-fA-F-]{36})\)\)").unwrap();
    re.captures_iter(text)
        .filter_map(|cap| cap.get(1).map(|m| m.as_str().to_owned()))
        .collect()
}

/// Very basic "parse" for markdown + math
pub fn parse_markdown(text: &str) -> String {
    // This would use comrak/pulldown-cmark in real app
    // For now, just return the string as-is
    text.replace("$$", "<math-block>")
        .replace("$", "<math-inline>")
        // ...extend as needed
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_block_refs() {
        let txt = "See ((123e4567-e89b-12d3-a456-426614174000)) and ((111e4567-e89b-12d3-a456-426614174001)).";
        let refs = extract_block_refs(txt);
        assert_eq!(refs.len(), 2);
    }
}