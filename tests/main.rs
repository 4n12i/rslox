mod tests {
    extern crate rslox;

    use rslox::lox::Lox;

    #[test]
    fn check_assignment() {
        assert!(Lox::run_file("examples/assignment/err_grouping.lox").is_err());
        assert!(Lox::run_file("examples/assignment/err_infix_operator.lox").is_err());
        assert!(Lox::run_file("examples/assignment/err_prefix_operator.lox").is_err());
        assert!(Lox::run_file("examples/assignment/err_to_this.lox").is_err());
        assert!(Lox::run_file("examples/assignment/err_undefined.lox").is_err());
        assert!(Lox::run_file("examples/assignment/ok_associativity.lox").is_ok());
        assert!(Lox::run_file("examples/assignment/ok_global.lox").is_ok());
        assert!(Lox::run_file("examples/assignment/ok_local.lox").is_ok());
        assert!(Lox::run_file("examples/assignment/ok_syntax.lox").is_ok());
    }

    #[test]
    fn check_block() {
        assert!(Lox::run_file("examples/block/ok_empty.lox").is_ok());
        assert!(Lox::run_file("examples/block/ok_scope.lox").is_ok());
    }

    #[test]
    fn check_bool() {
        assert!(Lox::run_file("examples/bool/ok_equality.lox").is_ok());
        assert!(Lox::run_file("examples/bool/ok_not.lox").is_ok());
    }

    #[test]
    fn check_call() {
        assert!(Lox::run_file("examples/call/err_bool.lox").is_err());
        assert!(Lox::run_file("examples/call/err_nil.lox").is_err());
        assert!(Lox::run_file("examples/call/err_num.lox").is_err());
        assert!(Lox::run_file("examples/call/err_object.lox").is_err());
        assert!(Lox::run_file("examples/call/err_string.lox").is_err());
    }

    #[test]
    fn check_closure() {
        assert!(Lox::run_file("examples/closure/ok_assign_to_closure.lox").is_ok());
        assert!(Lox::run_file("examples/closure/ok_assign_to_shadowed_later.lox").is_ok());
        assert!(Lox::run_file("examples/closure/ok_close_over_function_parameter.lox").is_ok());
        assert!(Lox::run_file("examples/closure/ok_close_over_later_variable.lox").is_ok());
        // assert!(Lox::run_file("examples/closure/ok_close_over_method_parameter.lox").is_ok());
        assert!(Lox::run_file("examples/closure/ok_closed_closure_in_function.lox").is_ok());
        assert!(Lox::run_file("examples/closure/ok_nested_closure.lox").is_ok());
        assert!(Lox::run_file("examples/closure/ok_open_closure_in_function.lox").is_ok());
        assert!(Lox::run_file("examples/closure/ok_reference_closure_multiple_times.lox").is_ok());
        assert!(Lox::run_file("examples/closure/ok_reuse_closure_slot.lox").is_ok());
        assert!(Lox::run_file("examples/closure/ok_shadow_closure_with_local.lox").is_ok());
        assert!(Lox::run_file("examples/closure/ok_unused_closure.lox").is_ok());
        assert!(Lox::run_file("examples/closure/ok_unused_later_closure.lox").is_ok());
    }

    #[test]
    fn check_comments() {
        assert!(Lox::run_file("examples/comments/ok_line_at_eof.lox").is_ok());
        assert!(Lox::run_file("examples/comments/ok_only_line_comment.lox").is_ok());
        assert!(Lox::run_file("examples/comments/ok_only_line_comment_and_line.lox").is_ok());
        assert!(Lox::run_file("examples/comments/ok_unicode.lox").is_ok());
    }

    #[test]
    fn check_for() {
        assert!(Lox::run_file("examples/for/err_class_in_body.lox").is_err());
        assert!(Lox::run_file("examples/for/err_fun_in_body.lox").is_err());
        assert!(Lox::run_file("examples/for/err_statement_condition.lox").is_err());
        assert!(Lox::run_file("examples/for/err_statement_increment.lox").is_err());
        assert!(Lox::run_file("examples/for/err_statement_initializer.lox").is_err());
        assert!(Lox::run_file("examples/for/err_var_in_body.lox").is_err());
        assert!(Lox::run_file("examples/for/ok_closure_in_body.lox").is_ok());
        assert!(Lox::run_file("examples/for/ok_return_closure.lox").is_ok());
        assert!(Lox::run_file("examples/for/ok_return_inside.lox").is_ok());
        assert!(Lox::run_file("examples/for/ok_scope.lox").is_ok());
        assert!(Lox::run_file("examples/for/ok_syntax.lox").is_ok());
    }

    #[test]
    fn check_function() {
        assert!(Lox::run_file("examples/function/err_body_must_be_block.lox").is_err());
        assert!(Lox::run_file("examples/function/err_extra_arguments.lox").is_err());
        // assert!(Lox::run_file("examples/function/err_local_mutual_recursion.lox").is_err());
        assert!(Lox::run_file("examples/function/err_missing_arguments.lox").is_err());
        assert!(Lox::run_file("examples/function/err_missing_comma_in_parameters.lox").is_err());
        assert!(Lox::run_file("examples/function/err_too_many_arguments.lox").is_err());
        assert!(Lox::run_file("examples/function/err_too_many_parameters.lox").is_err());
        assert!(Lox::run_file("examples/function/ok_empty_body.lox").is_ok());
        assert!(Lox::run_file("examples/function/ok_local_recursion.lox").is_ok());
        assert!(Lox::run_file("examples/function/ok_mutual_recursion.lox").is_ok());
        assert!(Lox::run_file("examples/function/ok_nested_call_with_arguments.lox").is_ok());
        assert!(Lox::run_file("examples/function/ok_parameters.lox").is_ok());
        assert!(Lox::run_file("examples/function/ok_print.lox").is_ok());
        assert!(Lox::run_file("examples/function/ok_recursion.lox").is_ok());
    }

    #[test]
    fn check_if() {
        assert!(Lox::run_file("examples/if/err_class_in_else.lox").is_err());
        assert!(Lox::run_file("examples/if/err_class_in_then.lox").is_err());
        assert!(Lox::run_file("examples/if/err_fun_in_else.lox").is_err());
        assert!(Lox::run_file("examples/if/err_fun_in_then.lox").is_err());
        assert!(Lox::run_file("examples/if/err_var_in_else.lox").is_err());
        assert!(Lox::run_file("examples/if/err_var_in_then.lox").is_err());
        assert!(Lox::run_file("examples/if/ok_dangling_else.lox").is_ok());
        assert!(Lox::run_file("examples/if/ok_else.lox").is_ok());
        assert!(Lox::run_file("examples/if/ok_if.lox").is_ok());
        assert!(Lox::run_file("examples/if/ok_truth.lox").is_ok());
    }

    #[test]
    fn check_logical_operator() {
        assert!(Lox::run_file("examples/logical_operator/ok_and.lox").is_ok());
        assert!(Lox::run_file("examples/logical_operator/ok_and_truth.lox").is_ok());
        assert!(Lox::run_file("examples/logical_operator/ok_or.lox").is_ok());
        assert!(Lox::run_file("examples/logical_operator/ok_or_truth.lox").is_ok());
    }

    #[test]
    fn check_nil() {
        assert!(Lox::run_file("examples/nil/ok_literal.lox").is_ok());
    }

    #[test]
    fn check_number() {
        assert!(Lox::run_file("examples/number/err_decimal_point_at_eof.lox").is_err());
        assert!(Lox::run_file("examples/number/err_leading_dot.lox").is_err());
        assert!(Lox::run_file("examples/number/err_trailing_dot.lox").is_err());
        assert!(Lox::run_file("examples/number/ok_literals.lox").is_ok());
        assert!(Lox::run_file("examples/number/ok_nan_equality.lox").is_ok());
    }

    #[test]
    fn check_operator() {
        assert!(Lox::run_file("examples/operator/err_add_bool_nil.lox").is_err());
        assert!(Lox::run_file("examples/operator/err_add_bool_num.lox").is_err());
        assert!(Lox::run_file("examples/operator/err_add_bool_string.lox").is_err());
        assert!(Lox::run_file("examples/operator/err_add_nil_nil.lox").is_err());
        assert!(Lox::run_file("examples/operator/err_add_num_nil.lox").is_err());
        assert!(Lox::run_file("examples/operator/err_add_string_nil.lox").is_err());
        assert!(Lox::run_file("examples/operator/err_divide_nonnum_num.lox").is_err());
        assert!(Lox::run_file("examples/operator/err_divide_num_nonnum.lox").is_err());
        assert!(Lox::run_file("examples/operator/err_greater_nonnum_num.lox").is_err());
        assert!(Lox::run_file("examples/operator/err_greater_num_nonnum.lox").is_err());
        assert!(Lox::run_file("examples/operator/err_greater_or_equal_nonnum_num.lox").is_err());
        assert!(Lox::run_file("examples/operator/err_greater_or_equal_num_nonnum.lox").is_err());
        assert!(Lox::run_file("examples/operator/err_less_nonnum_num.lox").is_err());
        assert!(Lox::run_file("examples/operator/err_less_num_nonnum.lox").is_err());
        assert!(Lox::run_file("examples/operator/err_less_or_equal_nonnum_num.lox").is_err());
        assert!(Lox::run_file("examples/operator/err_less_or_equal_num_nonnum.lox").is_err());
        assert!(Lox::run_file("examples/operator/err_multiply_nonnum_num.lox").is_err());
        assert!(Lox::run_file("examples/operator/err_multiply_num_nonnum.lox").is_err());
        assert!(Lox::run_file("examples/operator/err_negate_nonnum.lox").is_err());
        assert!(Lox::run_file("examples/operator/err_subtract_nonnum_num.lox").is_err());
        assert!(Lox::run_file("examples/operator/err_subtract_num_nonnum.lox").is_err());
        assert!(Lox::run_file("examples/operator/ok_add.lox").is_ok());
        assert!(Lox::run_file("examples/operator/ok_comparison.lox").is_ok());
        assert!(Lox::run_file("examples/operator/ok_divide.lox").is_ok());
        assert!(Lox::run_file("examples/operator/ok_equals.lox").is_ok());
        // assert!(Lox::run_file("examples/operator/ok_equals_class.lox").is_ok());
        // assert!(Lox::run_file("examples/operator/ok_equals_method.lox").is_ok());
        assert!(Lox::run_file("examples/operator/ok_multiply.lox").is_ok());
        assert!(Lox::run_file("examples/operator/ok_negate.lox").is_ok());
        assert!(Lox::run_file("examples/operator/ok_not.lox").is_ok());
        // assert!(Lox::run_file("examples/operator/ok_not_class.lox").is_ok());
        assert!(Lox::run_file("examples/operator/ok_not_equals.lox").is_ok());
        assert!(Lox::run_file("examples/operator/ok_subtract.lox").is_ok());
    }

    #[test]
    fn check_print() {
        assert!(Lox::run_file("examples/print/err_missing_argument.lox").is_err());
    }

    #[test]
    fn check_return() {
        assert!(Lox::run_file("examples/return/err_at_top_level.lox").is_err());
        assert!(Lox::run_file("examples/return/ok_after_else.lox").is_ok());
        assert!(Lox::run_file("examples/return/ok_after_if.lox").is_ok());
        assert!(Lox::run_file("examples/return/ok_after_while.lox").is_ok());
        assert!(Lox::run_file("examples/return/ok_in_function.lox").is_ok());
        // assert!(Lox::run_file("examples/return/ok_in_method.lox").is_ok());
        assert!(Lox::run_file("examples/return/ok_return_nil_if_no_value.lox").is_ok());
    }

    #[test]
    fn check_string() {
        assert!(Lox::run_file("examples/string/err_error_after_multiline.lox").is_err());
        assert!(Lox::run_file("examples/string/err_unterminated.lox").is_err());
        assert!(Lox::run_file("examples/string/ok_literals.lox").is_ok());
        assert!(Lox::run_file("examples/string/ok_multiline.lox").is_ok());
    }

    #[test]
    fn check_variable() {
        // assert!(Lox::run_file("examples/variable/err_collide_with_parameter.lox").is_err());
        // assert!(Lox::run_file("examples/variable/err_duplicate_local.lox").is_err());
        // assert!(Lox::run_file("examples/variable/err_duplicate_parameter.lox").is_err());
        assert!(Lox::run_file("examples/variable/err_undefined_global.lox").is_err());
        assert!(Lox::run_file("examples/variable/err_undefined_local.lox").is_err());
        assert!(Lox::run_file("examples/variable/err_use_false_as_var.lox").is_err());
        // assert!(Lox::run_file("examples/variable/err_use_local_in_initializer.lox").is_err());
        assert!(Lox::run_file("examples/variable/err_use_nil_as_var.lox").is_err());
        assert!(Lox::run_file("examples/variable/err_use_this_as_var.lox").is_err());
        assert!(Lox::run_file("examples/variable/ok_early_bound.lox").is_ok());
        assert!(Lox::run_file("examples/variable/ok_in_middle_of_block.lox").is_ok());
        assert!(Lox::run_file("examples/variable/ok_in_nested_block.lox").is_ok());
        // assert!(Lox::run_file("examples/variable/ok_local_from_method.lox").is_ok());
        assert!(Lox::run_file("examples/variable/ok_redeclare_global.lox").is_ok());
        assert!(Lox::run_file("examples/variable/ok_redefine_global.lox").is_ok());
        assert!(Lox::run_file("examples/variable/ok_scope_reuse_in_different_blocks.lox").is_ok());
        assert!(Lox::run_file("examples/variable/ok_shadow_and_local.lox").is_ok());
        assert!(Lox::run_file("examples/variable/ok_shadow_global.lox").is_ok());
        assert!(Lox::run_file("examples/variable/ok_shadow_local.lox").is_ok());
        assert!(Lox::run_file("examples/variable/ok_uninitialized.lox").is_ok());
        assert!(Lox::run_file("examples/variable/ok_unreached_undefined.lox").is_ok());
        assert!(Lox::run_file("examples/variable/ok_use_global_in_initializer.lox").is_ok());
    }

    #[test]
    fn check_while() {
        assert!(Lox::run_file("examples/while/err_class_in_body.lox").is_err());
        assert!(Lox::run_file("examples/while/err_fun_in_body.lox").is_err());
        assert!(Lox::run_file("examples/while/err_var_in_body.lox").is_err());
        assert!(Lox::run_file("examples/while/ok_closure_in_body.lox").is_ok());
        assert!(Lox::run_file("examples/while/ok_return_closure.lox").is_ok());
        assert!(Lox::run_file("examples/while/ok_return_inside.lox").is_ok());
        assert!(Lox::run_file("examples/while/ok_syntax.lox").is_ok());
    }

    #[test]
    fn check_others() {
        assert!(Lox::run_file("examples/err_unexpected_character.lox").is_err());
        assert!(Lox::run_file("examples/ok_debug.lox").is_ok());
        assert!(Lox::run_file("examples/ok_empty_file.lox").is_ok());
        assert!(Lox::run_file("examples/ok_fibonacci.lox").is_ok());
        assert!(Lox::run_file("examples/ok_precedence.lox").is_ok());
    }
}
