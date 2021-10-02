use crate::kernel::{Documentation, Parameter};
use pest::Parser;
use pest::Token;

/// Taken from <https://github.com/flying-sheep/rust-rst/blob/master/parser/src/rst.pest>
#[derive(Parser)]
#[grammar = "rst.pest"]
pub struct RstParser;

impl RstParser {
    pub fn parse_input(input: &str) -> Documentation {
        let rst_document = Self::parse(Rule::document, input).expect("unsuccessful parse");
        let mut kernel_parameters = Vec::new();
        let titles = rst_document
            .filter(|block| block.as_rule() == Rule::title)
            .map(|title| {
                (
                    title.as_str().lines().next().unwrap(),
                    title
                        .tokens()
                        .filter_map(|token| match token {
                            Token::Start { rule, pos } | Token::End { rule, pos } => {
                                if rule == Rule::title {
                                    Some(pos.pos())
                                } else {
                                    None
                                }
                            }
                        })
                        .collect::<Vec<usize>>(),
                )
            })
            .collect::<Vec<(&str, Vec<usize>)>>();
        for (i, (title, pos)) in titles.iter().enumerate() {
            assert_eq!(2, pos.len());
            if let Some(next_title) = titles.get(i + 1) {
                kernel_parameters.push(Parameter::new(
                    *title,
                    (input[pos[1]..next_title.1[0]]).as_ref(),
                ));
            } else {
                kernel_parameters.push(Parameter::new(*title, (input[pos[1]..]).as_ref()));
            };
        }
        Documentation::new(kernel_parameters)
    }
}
