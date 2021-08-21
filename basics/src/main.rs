mod basic_concepts;
mod compound_types;
mod control_flow;
mod functions;
mod scalar_types;
mod simple_types;
mod variables;

fn main() {
    basic_concepts::main();
    variables::main();
    functions::main();
    simple_types::main();
    scalar_types::main();
    compound_types::main();
    control_flow::main();
}
