#[cfg(test)]
mod rukinia_false {

    use rukinia::core::rukinia_result::RukiniaResultType;
    use rukinia::core::syntax::{Syntax, SyntaxForTrait};
    use rukinia::tasks::bool::rukinia_false::RukiniaFalse;
    use rukinia::tasks::task::RukiniaProcess;

    #[test]
    fn test_rukinia_false() {
        let result = RukiniaFalse::new(
            Vec::new(),
            SyntaxForTrait::Arguments(Vec::new()),
        );
        assert!(result.is_ok());
        let rukinia_cmd = result.unwrap();
        assert!(rukinia_cmd.get_result().result_type == RukiniaResultType::TestFail);
        assert!(rukinia_cmd.display_format() == "false");
    }

    #[test]
    fn test_rukinia_not_false() {
        let result = RukiniaFalse::new(
            Vec::new(),
            SyntaxForTrait::Arguments(vec![Syntax::Not]),
        );
        assert!(result.is_ok());
        let rukinia_cmd = result.unwrap();
        assert!(rukinia_cmd.get_result().result_type == RukiniaResultType::TestSuccess);
        assert!(rukinia_cmd.display_format() == "not false");
    }

    #[test]
    fn test_rukinia_false_get_command() {
        assert_eq!(RukiniaFalse::get_rukinia_command(), "false");
    }
}
