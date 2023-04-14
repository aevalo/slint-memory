use std::env;
use slint_interpreter::{ComponentCompiler, Value, SharedString, ComponentHandle};

fn main() {
  let args: Vec<String> = env::args().collect();

  let code = r#"
  export component MyWin inherits Window {
    in property <string> my_name;
    Text {
      text: "Hello, " + my_name;
    }
  }
  "#;
  
  let mut compiler = ComponentCompiler::default();
  let definition = spin_on::spin_on(compiler.build_from_source(code.into(), Default::default()));
  assert!(compiler.diagnostics().is_empty());
  let instance = definition.unwrap().create().unwrap();
  if let Some(name) = args.get(1) {
    instance.set_property("my_name", Value::from(SharedString::from(name))).unwrap();
  } else {
    instance.set_property("my_name", Value::from(SharedString::from("World"))).unwrap();
  }
  instance.run().unwrap();
}
