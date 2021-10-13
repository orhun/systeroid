use crate::parser::Rule;
use lazy_regex::{regex, Lazy, Regex};
use pest::iterators::Pair;
use pest::Token;
use std::convert::TryFrom;
use systeroid_core::error::Error as ErrorImpl;

/// Regex for matching the explanation of the sysctl sections.
///
/// These _titles_ should be skipped since they are often describing the
/// documentation in the following section rather than a kernel parameter.
///
/// e.g. `2. /proc/sys/fs/binfmt_misc`
static SECTION_EXPL_REGEX: &Lazy<Regex> = regex!("[0-9].\\s/proc/sys/");

/// Title from the parsed RST document.
#[derive(Debug, Default)]
pub struct Title<'a> {
    /// Title value.
    pub value: &'a str,
    /// Start position of the title.
    pub start_pos: usize,
    /// End position of the title.
    pub end_pos: usize,
}

impl<'a> TryFrom<Pair<'a, Rule>> for Title<'a> {
    type Error = ErrorImpl;

    fn try_from(pair: Pair<'a, Rule>) -> Result<Self, Self::Error> {
        let mut title = Title::default();

        // check if the rule matches
        if pair.as_rule() != Rule::title {
            return Err(ErrorImpl::ParseError(String::from(
                "parsed section is not a title",
            )));
        }

        // set the actual title
        if let Some(value) = pair.as_str().lines().next() {
            if value.chars().all(|v| v == '=') {
                return Err(ErrorImpl::ParseError(String::from(
                    "document beginning found",
                )));
            } else if SECTION_EXPL_REGEX.is_match(value) {
                return Err(ErrorImpl::ParseError(String::from(
                    "section explanation found",
                )));
            }
            title.value = value;
        } else {
            return Err(ErrorImpl::ParseError(String::from("invalid title")));
        }

        // set token positions
        pair.tokens().for_each(|token| match token {
            Token::Start { rule, pos } => {
                if rule == Rule::title {
                    title.start_pos = pos.pos();
                }
            }
            Token::End { rule, pos } => {
                if rule == Rule::title {
                    title.end_pos = pos.pos();
                }
            }
        });

        Ok(title)
    }
}
