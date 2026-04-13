//! Aiken grammar for tree-sitter.
use tree_sitter_language::LanguageFn;

extern "C" {
    fn tree_sitter_aiken() -> *const ();
}

/// The tree-sitter [`LanguageFn`] for Aiken.
pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_aiken) };

#[cfg(test)]
mod tests {
    #[test]
    fn test_can_load_grammar() {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&super::LANGUAGE.into())
            .expect("Error loading Aiken grammar");
    }
}
