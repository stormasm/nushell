use nu_plugin::{serve_plugin, EvaluatedCall, JsonSerializer, LabeledError, Plugin};
use nu_protocol::{PluginSignature, Type, Value};

struct Len;

impl Len {
    fn new() -> Self {
        Self
    }
}

impl Plugin for Len {
    fn signature(&self) -> Vec<PluginSignature> {
        vec![PluginSignature::build("len")
            .usage("calculates the length of its input")
            .input_output_types(vec![(Type::String, Type::Int)])]
    }

    fn run(
        &mut self,
        name: &str,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        assert_eq!(name, "len");
        match input {
            Value::String { val, .. } => Ok(Value::int(val.len() as i64, call.head)),
            _ => Err(LabeledError {
                label: "Expected String input from pipeline".to_string(),
                msg: format!("requires string input; got {}", input.get_type()),
                span: Some(call.head),
            }),
        }
    }
}

fn main() {
    serve_plugin(&mut Len::new(), JsonSerializer)
}
