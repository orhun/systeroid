use owo_colors::colored::Color;
use owo_colors::{OwoColorize, Stream::Stdout};
use std::fmt::Display;
use std::io::{Result as IoResult, Write};

/// Vertical connector.
const VERTICAL_STR: &str = "│";
/// Horizontal connector.
const HORIZONTAL_STR: &str = "├──";
/// Horizontal connector for the last node.
const LAST_HORIZONTAL_STR: &str = "└──";

/// Representation of a tree node.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct TreeNode {
    /// Value of the node.
    value: String,
    /// Children of the node.
    pub children: Vec<TreeNode>,
}

impl TreeNode {
    /// Adds new child nodes to the tree node.
    pub fn add<'a, I>(&mut self, values: &mut I)
    where
        I: Iterator<Item = &'a str>,
    {
        if let Some(value) = values.next() {
            let mut found = false;
            for child in self.children.iter_mut() {
                if &*child.value == value {
                    child.add(values);
                    found = true;
                    break;
                }
            }
            if !found {
                let new_child = TreeNode {
                    value: value.to_string(),
                    children: Vec::new(),
                };
                self.children.push(new_child);
                if let Some(last_child) = self.children.last_mut() {
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
        connector_color: Color,
    ) -> IoResult<()> {
        self.print_line(out, &connectors[..], connector_color)?;
        connectors.push(false);
        for (i, child) in self.children.iter().enumerate() {
            if self.children.len() == i + 1 {
                if let Some(last_connector) = connectors.last_mut() {
                    *last_connector = true;
                }
            }
            child.print(out, connectors, connector_color)?;
        }
        connectors.pop();
        Ok(())
    }

    /// Prints a single line with the given connectors.
    fn print_line<Output: Write>(
        &self,
        output: &mut Output,
        connectors: &[bool],
        connector_color: Color,
    ) -> IoResult<()> {
        if let Some(last_connector) = connectors.last() {
            for last in &connectors[..connectors.len() - 1] {
                write!(
                    output,
                    "{}   ",
                    if *last { " " } else { VERTICAL_STR }
                        .if_supports_color(Stdout, |v| v.color(connector_color))
                )?;
            }
            if *last_connector {
                write!(
                    output,
                    "{} ",
                    LAST_HORIZONTAL_STR.if_supports_color(Stdout, |v| v.color(connector_color))
                )?;
            } else {
                write!(
                    output,
                    "{} ",
                    HORIZONTAL_STR.if_supports_color(Stdout, |v| v.color(connector_color))
                )?;
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
    pub fn from_input<I, O>(input: &mut I, separator: char) -> Self
    where
        I: Iterator<Item = O>,
        O: Display,
    {
        let mut root = TreeNode::default();
        for line in input.map(|v| v.to_string()) {
            let mut components = line.split(separator);
            root.add(&mut components);
        }
        Self::new(root.children)
    }

    /// Prints the full tree to the given output.
    pub fn print<Output: Write>(&self, out: &mut Output, connector_color: Color) -> IoResult<()> {
        for node in &self.nodes {
            node.print(out, &mut Vec::new(), connector_color)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_single_tree_creation(lines: &[&str], separator: char, expected_tree: TreeNode) {
        let tree = Tree::from_input(&mut lines.iter(), separator);
        assert_eq!(1, tree.nodes.len());
        assert_eq!(expected_tree, tree.nodes[0]);
    }

    #[test]
    fn test_tree_creation1() {
        let lines = ["a", "a/b", "a/b/c/d", "a/b/e"];
        let e = TreeNode {
            value: String::from("e"),
            children: Vec::new(),
        };
        let d = TreeNode {
            value: String::from("d"),
            children: Vec::new(),
        };
        let c = TreeNode {
            value: String::from("c"),
            children: vec![d],
        };
        let b = TreeNode {
            value: String::from("b"),
            children: vec![c, e],
        };
        let expected_tree = TreeNode {
            value: String::from("a"),
            children: vec![b],
        };

        test_single_tree_creation(&lines, '/', expected_tree);
    }

    #[test]
    fn test_tree_creation2() {
        let lines = ["a", "a/b/e", "a/b", "a/b/c/d"];
        let e = TreeNode {
            value: String::from("e"),
            children: Vec::new(),
        };
        let d = TreeNode {
            value: String::from("d"),
            children: Vec::new(),
        };
        let c = TreeNode {
            value: String::from("c"),
            children: vec![d],
        };
        let b = TreeNode {
            value: String::from("b"),
            children: vec![e, c],
        };
        let expected_tree = TreeNode {
            value: String::from("a"),
            children: vec![b],
        };

        test_single_tree_creation(&lines, '/', expected_tree);
    }

    #[test]
    fn test_trees_creation() {
        let lines = ["a", "a/b", "c/d"];
        let d = TreeNode {
            value: String::from("d"),
            children: Vec::new(),
        };
        let c = TreeNode {
            value: String::from("c"),
            children: vec![d],
        };
        let b = TreeNode {
            value: String::from("b"),
            children: Vec::new(),
        };
        let a = TreeNode {
            value: String::from("a"),
            children: vec![b],
        };

        let tree = Tree::from_input(&mut lines.iter(), '/');
        assert_eq!(2, tree.nodes.len());
        assert_eq!(a, tree.nodes[0]);
        assert_eq!(c, tree.nodes[1]);
    }

    #[test]
    fn test_tree_output() -> IoResult<()> {
        let lines = ["a", "a/b/e", "a/b", "a/b/c/d"];

        let tree = Tree::from_input(&mut lines.iter(), '/');
        let mut output = Vec::new();
        tree.print(&mut output, Color::White)?;

        let expected_output = "\
a
└── b
    ├── e
    └── c
        └── d\n";

        assert_eq!(expected_output, String::from_utf8_lossy(&output));

        Ok(())
    }

    #[test]
    fn test_print_line() -> IoResult<()> {
        let value = String::from("abc\ndef");

        let mut output = Vec::new();
        TreeNode {
            value: value.to_string(),
            children: Vec::new(),
        }
        .print_line(&mut output, &[], Color::White)?;
        assert_eq!(b"abc\ndef\n", &*output);

        let mut output = Vec::new();
        TreeNode {
            value: value.to_string(),
            children: Vec::new(),
        }
        .print_line(&mut output, &[true, false, true], Color::White)?;
        assert_eq!("    │   └── abc\ndef\n".as_bytes(), &*output);

        let mut output = Vec::new();
        TreeNode {
            value,
            children: Vec::new(),
        }
        .print_line(&mut output, &[true, false, false], Color::White)?;
        assert_eq!("    │   ├── abc\ndef\n".as_bytes(), &*output);

        Ok(())
    }
}
