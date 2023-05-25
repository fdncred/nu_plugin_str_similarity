use nu_plugin::LabeledError;
use nu_protocol::{Span, Spanned, Value};
use textdistance::{nstr, str};

pub fn str_similarity_do_something(
    compare_to_str: Spanned<String>,
    normalize: bool,
    input_val: &str,
    input_span: Span,
) -> Result<Value, LabeledError> {
    let compare_from = input_val;
    let compare_to = compare_to_str.item;

    let a_val = if normalize {
        format!("Comparing {compare_from} to {compare_to} with norm {normalize:?}")
    } else {
        format!("Comparing {compare_from} to {compare_to} with normalize {normalize:?}")
    };

    // let a_val = match normalize {
    //     Some(p) => format!("Hello, {}! with value: {}", p.item, input_val),
    //     None => format!("Hello, Default! with value: {}", input_val),
    // };
    Ok(Value::String {
        val: a_val,
        span: input_span,
    })
}
