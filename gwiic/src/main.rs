use libgwiic::Compiler;

fn main() {
    let source = include_str!("../test.gwii");
    println!("---------------------------SOURCE---------------------------");
    println!("{}", source);
    println!("--------------------------AST-NODE--------------------------");
    let mut compiler = Compiler::new();
    let source_file = compiler.create_virtual_source("/gwii/repl", source);
    let node_list = source_file.parse();
    for node in node_list {
        println!("{:#?}", node);
    }
}
