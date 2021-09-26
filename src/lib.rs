pub mod reader;

use std::path::PathBuf;

pub fn run() {
    let kernel_docs = PathBuf::from("/usr/share/doc/linux");
    let sysctl_docs = kernel_docs.join("admin-guide").join("sysctl");
    let kernel_section = reader::read_to_string(&sysctl_docs.join("kernel.rst")).unwrap();
    println!("{}", kernel_section);
}
