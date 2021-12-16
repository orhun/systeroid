use std::fmt::Display;
use std::io::{Result as IoResult, Write};

/// Vertical character for connecting sequential nodes.
const VERTICAL_CHAR: char = '│';
/// Horizontal connector.
const HORIZONTAL_STR: &str = "├──";
/// Horizontal connector for the last node.
const LAST_HORIZONTAL_STR: &str = "└──";

/// Representation of a tree node.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct TreeNode {
    /// Value of the node.
    value: String,
    /// Childs of the node.
    pub childs: Vec<TreeNode>,
}

impl TreeNode {
    /// Adds new child nodes to the tree node.
    pub fn add<'a, I: Iterator<Item = &'a str>>(&mut self, values: &mut I) {
        if let Some(value) = values.next() {
            let mut found = false;
            for child in self.childs.iter_mut() {
                if &*child.value == value {
                    child.add(values);
                    found = true;
                    break;
                }
            }
            if !found {
                let new_child = TreeNode {
                    value: value.to_string(),
                    childs: Vec::new(),
                };
                self.childs.push(new_child);
                if let Some(last_child) = self.childs.last_mut() {
                    last_child.add(values);
                }
            }
        }
    }

    /// Prints the node to the given output.
    pub fn print<Output: Write>(
        &self,
        out: &mut Output,
        connectors: &mut Vec<bool>,
    ) -> IoResult<()> {
        self.print_line(out, &connectors[..])?;
        connectors.push(false);
        for (i, child) in self.childs.iter().enumerate() {
            if self.childs.len() == i + 1 {
                if let Some(last_connector) = connectors.last_mut() {
                    *last_connector = true;
                }
            }
            child.print(out, connectors)?;
        }
        connectors.pop();
        Ok(())
    }

    /// Prints a single line with the given connectors.
    fn print_line<Output: Write>(&self, output: &mut Output, connectors: &[bool]) -> IoResult<()> {
        if let Some(last_connector) = connectors.last() {
            for last in &connectors[..connectors.len() - 1] {
                write!(output, "{}   ", if *last { ' ' } else { VERTICAL_CHAR })?;
            }
            if *last_connector {
                write!(output, "{} ", LAST_HORIZONTAL_STR)?;
            } else {
                write!(output, "{} ", HORIZONTAL_STR)?;
            }
        }
        writeln!(output, "{}", self.value)?;
        Ok(())
    }
}

/// Representation of a tree structure.
#[derive(Debug)]
pub struct Tree {
    /// Nodes of the tree.
    nodes: Vec<TreeNode>,
}

impl Tree {
    /// Constructs a new instance.
    pub fn new(nodes: Vec<TreeNode>) -> Self {
        Self { nodes }
    }

    /// Constructs a new instance from given input.
    pub fn from_input<I, O>(input: &mut I, seperator: char) -> Self
    where
        I: Iterator<Item = O>,
        O: Display,
    {
        let mut root = TreeNode::default();
        for line in input.map(|v| v.to_string()) {
            let mut components = line.split(seperator);
            root.add(&mut components);
        }
        Self::new(root.childs)
    }

    /// Prints the full tree to the given output.
    pub fn print<Output: Write>(&self, out: &mut Output) -> IoResult<()> {
        for node in &self.nodes {
            node.print(out, &mut Vec::new())?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_single_tree_creation(lines: &[&str], seperator: char, expected_tree: TreeNode) {
        let tree = Tree::from_input(&mut lines.iter(), seperator);
        assert_eq!(1, tree.nodes.len());
        assert_eq!(expected_tree, tree.nodes[0]);
    }

    #[test]
    fn test_tree_creation1() {
        let lines = ["a", "a/b", "a/b/c/d", "a/b/e"];
        let e = TreeNode {
            value: String::from("e"),
            childs: Vec::new(),
        };
        let d = TreeNode {
            value: String::from("d"),
            childs: Vec::new(),
        };
        let c = TreeNode {
            value: String::from("c"),
            childs: vec![d],
        };
        let b = TreeNode {
            value: String::from("b"),
            childs: vec![c, e],
        };
        let expected_tree = TreeNode {
            value: String::from("a"),
            childs: vec![b],
        };

        test_single_tree_creation(&lines, '/', expected_tree);
    }

    #[test]
    fn test_tree_creation2() {
        let lines = ["a", "a/b/e", "a/b", "a/b/c/d"];
        let e = TreeNode {
            value: String::from("e"),
            childs: Vec::new(),
        };
        let d = TreeNode {
            value: String::from("d"),
            childs: Vec::new(),
        };
        let c = TreeNode {
            value: String::from("c"),
            childs: vec![d],
        };
        let b = TreeNode {
            value: String::from("b"),
            childs: vec![e, c],
        };
        let expected_tree = TreeNode {
            value: String::from("a"),
            childs: vec![b],
        };

        test_single_tree_creation(&lines, '/', expected_tree);
    }

    #[test]
    fn test_trees_creation() {
        let lines = ["a", "a/b", "c/d"];
        let d = TreeNode {
            value: String::from("d"),
            childs: Vec::new(),
        };
        let c = TreeNode {
            value: String::from("c"),
            childs: vec![d],
        };
        let b = TreeNode {
            value: String::from("b"),
            childs: Vec::new(),
        };
        let a = TreeNode {
            value: String::from("a"),
            childs: vec![b],
        };

        let tree = Tree::from_input(&mut lines.iter(), '/');
        assert_eq!(2, tree.nodes.len());
        assert_eq!(a, tree.nodes[0]);
        assert_eq!(c, tree.nodes[1]);
    }

    #[test]
    fn test_print_line() {
        let value = String::from("abc\ndef");

        let mut output = Vec::new();
        TreeNode {
            value: value.to_string(),
            childs: Vec::new(),
        }
        .print_line(&mut output, &[])
        .unwrap();
        assert_eq!(b"abc\ndef\n", &*output);

        let mut output = Vec::new();
        TreeNode {
            value: value.to_string(),
            childs: Vec::new(),
        }
        .print_line(&mut output, &[true, false, true])
        .unwrap();
        assert_eq!("    │   └── abc\ndef\n".as_bytes(), &*output);

        let mut output = Vec::new();
        TreeNode {
            value: value.to_string(),
            childs: Vec::new(),
        }
        .print_line(&mut output, &[true, false, false])
        .unwrap();
        assert_eq!("    │   ├── abc\ndef\n".as_bytes(), &*output);
    }
}
