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
        vec![PluginSignature::build("str similarity")
            .usage("Compare strings to find similarity by algorithm")
            .required("string", SyntaxShape::String, "String to compare with")
            .switch(
                "normalize",
                "Normalize the results between 0 and 1",
                Some('n'),
            )
            .switch("list", "List all available algorithms", Some('l'))
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
        assert_eq!(name, "str similarity");
        let compare_to_str_optn: Option<Spanned<String>> = call.opt(0)?;
        let compare_to_str = match compare_to_str_optn {
            Some(p) => p,
            None => {
                return Err(LabeledError {
                    label: "Expected a string as a parameter".into(),
                    msg: format!("found nothing"),
                    span: Some(call.head),
                })
            }
        };
        let normalize = call.has_flag("normalize");
        let list = call.has_flag("list");

        let ret_val = match input {
            Value::String {
                val: input_val,
                span: input_span,
            } => crate::str_similarity::str_similarity_do_something(
                compare_to_str,
                normalize,
                input_val,
                *input_span,
            )?,
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
