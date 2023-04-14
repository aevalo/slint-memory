use slint_interpreter::{ComponentCompiler, ComponentHandle};

fn main() {
  let mut compiler = ComponentCompiler::default();
  let definition = spin_on::spin_on(compiler.build_from_path("ui/hello.slint"));
  slint_interpreter::print_diagnostics(&compiler.diagnostics());
  if let Some(definition) = definition {
      let instance = definition.create().unwrap();
      instance.run().unwrap();
  }
}
