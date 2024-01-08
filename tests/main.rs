#[cfg(test)]
mod tests {
    extern crate rslox;

    use rslox::lox::Lox;

    #[test]
    fn block() {
        // assert!(Lox::run_file("examples/block/ok_empty.lox").is_ok());
        assert!(Lox::run_file("examples/block/ok_scope.lox").is_ok());
    }

    #[test]
    fn bool() {
        assert!(Lox::run_file("examples/bool/ok_equality.lox").is_ok());
        assert!(Lox::run_file("examples/bool/ok_not.lox").is_ok());
    }

    #[test]
    fn comments() {
        assert!(Lox::run_file("examples/comments/ok_line_at_eof.lox").is_ok());
        assert!(Lox::run_file("examples/comments/ok_only_line_comment.lox").is_ok());
        assert!(Lox::run_file("examples/comments/ok_only_line_comment_and_line.lox").is_ok());
        // assert!(Lox::run_file("examples/comments/ok_unicode.lox").is_ok());
    }

    #[test]
    fn nil() {
        assert!(Lox::run_file("examples/nil/ok_literal.lox").is_ok());
    }

    #[test]
    fn number() {
        assert!(Lox::run_file("examples/number/err_decimal_point_at_eof.lox").is_err());
        assert!(Lox::run_file("examples/number/err_leading_dot.lox").is_err());
        assert!(Lox::run_file("examples/number/err_trailing_dot.lox").is_err());
        assert!(Lox::run_file("examples/number/ok_literals.lox").is_ok());
        assert!(Lox::run_file("examples/number/ok_nan_equality.lox").is_ok());
    }

    #[test]
    fn print() {
        assert!(Lox::run_file("examples/print/err_missing_argument.lox").is_err());
    }

    #[test]
    fn string() {
        assert!(Lox::run_file("examples/string/err_error_after_multiline.lox").is_err());
        assert!(Lox::run_file("examples/string/err_unterminated.lox").is_err());
        // assert!(Lox::run_file("examples/string/ok_literals.lox").is_ok());
        assert!(Lox::run_file("examples/string/ok_multiline.lox").is_ok());
    }

    #[test]
    fn others() {
        assert!(Lox::run_file("examples/err_unexpected_character.lox").is_err());
        assert!(Lox::run_file("examples/ok_empty_file.lox").is_ok());
        // assert!(Lox::run_file("examples/ok_precedence.lox").is_ok());
    }
}
