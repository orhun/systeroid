use kparams_parser::parser::RstParser;
use kparams_parser::reader;
use std::path::PathBuf;

fn main() {
    let kernel_docs = PathBuf::from("/usr/share/doc/linux");
    let sysctl_docs = kernel_docs.join("admin-guide").join("sysctl");
    let kernel_section = reader::read_to_string(&sysctl_docs.join("kernel.rst")).unwrap();

    let kernel_section_docs = RstParser::parse_docs(&kernel_section);
    for kernel_parameter in kernel_section_docs.parameters {
        println!("## {}", kernel_parameter.name);
        println!("{}", kernel_parameter.description);
    }
}
