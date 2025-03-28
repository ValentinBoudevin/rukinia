#[cfg(test)]
mod rukinia_cmd {

    use rukinia::core::rukinia_result::RukiniaResultType;
    use rukinia::core::syntax::{Syntax, SyntaxForTrait};
    use rukinia::tasks::rukinia_cmd::RukiniaCmd;
    use rukinia::tasks::task::RukiniaProcess;

    #[test]
    fn test_rukinia_cmd_without_args() {
        let result = RukiniaCmd::new(Vec::new(), SyntaxForTrait::Arguments(Vec::new()));
        assert!(result.is_err());
    }

    #[test]
    fn test_rukinia_cmd_success() {
        let result = RukiniaCmd::new(
            vec!["ls".to_string()],
            SyntaxForTrait::Arguments(Vec::new()),
        );
        assert!(result.is_ok());
        let rukinia_cmd = result.unwrap();
        assert!(rukinia_cmd.get_result().result_type == RukiniaResultType::TestSuccess);
        assert!(rukinia_cmd.display_format() == "Running \"ls\" is successful");
    }

    #[test]
    fn test_rukinia_cmd_not_success() {
        let result = RukiniaCmd::new(
            vec!["ls".to_string()],
            SyntaxForTrait::Arguments(vec![Syntax::Not]),
        );
        assert!(result.is_ok());
        let rukinia_cmd = result.unwrap();
        assert!(rukinia_cmd.get_result().result_type == RukiniaResultType::TestFail);
        assert!(rukinia_cmd.display_format() == "Running \"ls\" is not successful");
    }

    #[test]
    fn test_rukinia_cmd_fail() {
        let result = RukiniaCmd::new(
            vec!["not_an_executable_in_the_system".to_string()],
            SyntaxForTrait::Arguments(Vec::new()),
        );
        assert!(result.is_ok());
        let rukinia_cmd = result.unwrap();
        assert!(rukinia_cmd.get_result().result_type == RukiniaResultType::TestFail);
        assert!(
            rukinia_cmd.display_format()
                == "Running \"not_an_executable_in_the_system\" is successful"
        );
    }

    #[test]
    fn test_rukinia_cmd_get_command() {
        assert_eq!(RukiniaCmd::get_rukinia_command(), "rukinia_cmd");
    }
}
