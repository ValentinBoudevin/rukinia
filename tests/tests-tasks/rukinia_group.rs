#[cfg(test)]
mod rukinia_group {

    use rukinia::core::rukinia_result::RukiniaResultType;
    use rukinia::core::syntax::{Syntax, SyntaxForTrait};
    use rukinia::tasks::user::rukinia_group::RukiniaGroup;
    use rukinia::tasks::task::RukiniaProcess;

    #[test]
    fn test_rukinia_cmd_without_args() {
        let result = RukiniaGroup::new(Vec::new(), SyntaxForTrait::Arguments(Vec::new()));
        assert!(result.is_err());
    }

    #[test]
    fn test_rukinia_cmd_get_command() {
        assert_eq!(RukiniaGroup::get_rukinia_command(), "rukinia_group");
    }
}
