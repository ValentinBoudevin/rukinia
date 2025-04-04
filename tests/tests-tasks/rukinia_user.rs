#[cfg(test)]
mod rukinia_user {

    use rukinia::core::rukinia_result::RukiniaResultType;
    use rukinia::core::syntax::{Syntax, SyntaxForTrait};
    use rukinia::tasks::rukinia_user::RukiniaUser;
    use rukinia::tasks::task::RukiniaProcess;

    #[test]
    fn test_rukinia_cmd_without_args() {
        let result = RukiniaUser::new(Vec::new(), SyntaxForTrait::Arguments(Vec::new()));
        assert!(result.is_err());
    }

    #[test]
    fn test_rukinia_cmd_success() {
        let result = RukiniaUser::new(
            vec!["root".to_string()],
            SyntaxForTrait::Arguments(Vec::new()),
        );
        assert!(result.is_ok());
        let rukinia_cmd = result.unwrap();
        assert!(rukinia_cmd.get_result().result_type == RukiniaResultType::TestSuccess);
        assert!(rukinia_cmd.display_format() == "Checking user root exists");
    }

    #[test]
    fn test_rukinia_cmd_not_success() {
        let result = RukiniaUser::new(
            vec!["root".to_string()],
            SyntaxForTrait::Arguments(vec![Syntax::Not]),
        );
        assert!(result.is_ok());
        let rukinia_cmd = result.unwrap();
        assert!(rukinia_cmd.get_result().result_type == RukiniaResultType::TestFail);
        assert!(rukinia_cmd.display_format() == "Checking user root not exists");
    }

    #[test]
    fn test_rukinia_cmd_fail() {
        let result = RukiniaUser::new(
            vec!["i_dont_exist".to_string()],
            SyntaxForTrait::Arguments(Vec::new()),
        );
        assert!(result.is_ok());
        let rukinia_cmd = result.unwrap();
        assert!(rukinia_cmd.get_result().result_type == RukiniaResultType::TestFail);
        assert!(
            rukinia_cmd.display_format()
                == "Checking user i_dont_exist exists"
        );
    }

    #[test]
    fn test_rukinia_cmd_get_command() {
        assert_eq!(RukiniaUser::get_rukinia_command(), "rukinia_user");
    }
}
