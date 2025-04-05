#[cfg(test)]
mod rukinia_true {

    use rukinia::core::rukinia_result::RukiniaResultType;
    use rukinia::core::syntax::{Syntax, SyntaxForTrait};
    use rukinia::tasks::bool::rukinia_true::RukiniaTrue;
    use rukinia::tasks::task::RukiniaProcess;

    #[test]
    fn test_rukinia_true() {
        let result = RukiniaTrue::new(
            Vec::new(),
            SyntaxForTrait::Arguments(Vec::new()),
        );
        assert!(result.is_ok());
        let rukinia_cmd = result.unwrap();
        assert!(rukinia_cmd.get_result().result_type == RukiniaResultType::TestSuccess);
        assert!(rukinia_cmd.display_format() == "true");
    }

    #[test]
    fn test_rukinia_not_true() {
        let result = RukiniaTrue::new(
            Vec::new(),
            SyntaxForTrait::Arguments(vec![Syntax::Not]),
        );
        assert!(result.is_ok());
        let rukinia_cmd = result.unwrap();
        assert!(rukinia_cmd.get_result().result_type == RukiniaResultType::TestFail);
        assert!(rukinia_cmd.display_format() == "not true");
    }

    #[test]
    fn test_rukinia_true_get_command() {
        assert_eq!(RukiniaTrue::get_rukinia_command(), "true");
    }
}
