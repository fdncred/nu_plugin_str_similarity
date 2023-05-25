mod str_similarity;
use nu_plugin::{serve_plugin, EvaluatedCall, LabeledError, MsgPackSerializer, Plugin};
use nu_protocol::{Category, PluginExample, PluginSignature, Spanned, SyntaxShape, Value};

struct StrSimilarity;

impl StrSimilarity {
    fn new() -> Self {
        Self {}
    }
}

impl Plugin for StrSimilarity {
    fn signature(&self) -> Vec<PluginSignature> {
        vec![PluginSignature::build("str_similarity")
            .usage("View str_similarity results")
            .required("path", SyntaxShape::String, "path to str_similarity input file")
            .category(Category::Experimental)
            .plugin_examples(vec![PluginExample {
                description: "This is the example descripion".into(),
                example: "some pipeline involving str_similarity".into(),
                result: None,
            }])]
    }

    fn run(
        &mut self,
        name: &str,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        assert_eq!(name, "str_similarity");
        let param: Option<Spanned<String>> = call.opt(0)?;

        let ret_val = match input {
            Value::String { val, span } => {
                crate::str_similarity::str_similarity_do_something(param, val, *span)?
            }
            v => {
                return Err(LabeledError {
                    label: "Expected something from pipeline".into(),
                    msg: format!("requires some input, got {}", v.get_type()),
                    span: Some(call.head),
                });
            }
        };

        Ok(ret_val)
    }
}

fn main() {
    serve_plugin(&mut StrSimilarity::new(), MsgPackSerializer);
}
