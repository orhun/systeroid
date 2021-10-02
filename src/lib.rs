pub mod kernel;
pub mod parser;
pub mod reader;
#[macro_use]
extern crate pest_derive;

use parser::RstParser;
use std::path::PathBuf;

pub fn run() {
    let kernel_docs = PathBuf::from("/usr/share/doc/linux");
    let sysctl_docs = kernel_docs.join("admin-guide").join("sysctl");
    let kernel_section = reader::read_to_string(&sysctl_docs.join("kernel.rst")).unwrap();

    let kernel_section_docs = RstParser::parse_input(&kernel_section);
    for kernel_parameter in kernel_section_docs.parameters {
        println!("## {}", kernel_parameter.name);
        println!("{}", kernel_parameter.description);
    }
}
