#[cfg(test)]
mod tests {
    use git_visual::{CommitNode, CommitTree};

    #[test]
    fn test_single_commit() {
        let commit_node = CommitNode {
            id: "abcd123".to_string(),
            message: "Initial commit".to_string(),
            parents: vec![],
        };
        let commit_tree = CommitTree {
            nodes: vec![commit_node],
        };

        let plantuml_output = commit_tree.to_plantuml_string();
        let expected_output = "@startuml\n\
                               abcd123 : Initial commit\n\
                               @enduml\n";

        assert_eq!(plantuml_output, expected_output);
    }

    #[test]
    fn test_commit_chain() {
        let commit_node1 = CommitNode {
            id: "abcd123".to_string(),
            message: "First commit".to_string(),
            parents: vec![],
        };
        let commit_node2 = CommitNode {
            id: "bcde234".to_string(),
            message: "Second commit".to_string(),
            parents: vec!["abcd123".to_string()],
        };
        let commit_tree = CommitTree {
            nodes: vec![commit_node1, commit_node2],
        };

        let plantuml_output = commit_tree.to_plantuml_string();
        let expected_output = "@startuml\n\
                               abcd123 : First commit\n\
                               abcd123 --> bcde234\n\
                               bcde234 : Second commit\n\
                               @enduml\n";

        assert_eq!(plantuml_output, expected_output);
    }

    #[test]
    fn test_branching_commits() {
        let commit_node1 = CommitNode {
            id: "abcd123".to_string(),
            message: "First commit".to_string(),
            parents: vec![],
        };
        let commit_node2 = CommitNode {
            id: "bcde234".to_string(),
            message: "Second commit".to_string(),
            parents: vec!["abcd123".to_string()],
        };
        let commit_node3 = CommitNode {
            id: "cdef345".to_string(),
            message: "Third commit".to_string(),
            parents: vec!["abcd123".to_string()],
        };
        let commit_node4 = CommitNode {
            id: "defg456".to_string(),
            message: "Merge commit".to_string(),
            parents: vec!["bcde234".to_string(), "cdef345".to_string()],
        };
        let commit_tree = CommitTree {
            nodes: vec![commit_node1, commit_node2, commit_node3, commit_node4],
        };

        let plantuml_output = commit_tree.to_plantuml_string();
        let expected_output = "@startuml\n\
                               abcd123 : First commit\n\
                               abcd123 --> bcde234\n\
                               bcde234 : Second commit\n\
                               abcd123 --> cdef345\n\
                               cdef345 : Third commit\n\
                               bcde234 --> defg456\n\
                               cdef345 --> defg456\n\
                               defg456 : Merge commit\n\
                               @enduml\n";

        assert_eq!(plantuml_output, expected_output);
    }
}