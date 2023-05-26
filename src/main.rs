use std::vec;

use nu_plugin::{serve_plugin, EvaluatedCall, LabeledError, MsgPackSerializer, Plugin};
use nu_protocol::{Category, PluginExample, PluginSignature, Span, Spanned, SyntaxShape, Value};
use textdistance::{nstr, str};

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
            .named(
                "algorithm",
                SyntaxShape::String,
                "Name of the algorithm to compute",
                Some('a'),
            )
            .switch("all", "Run all algorithms", Some('A'))
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
        if list {
            return Ok(list_algorithms());
        }
        let algo: Option<String> = call.get_flag("algorithm")?;
        let sim = match algo {
            Some(a) => a.to_string(),
            None => "levenshtein".to_string(),
        };
        let all = call.has_flag("all");

        let ret_val = match input {
            Value::String {
                val: input_val,
                span: input_span,
            } => {
                if all {
                    compute_all(&compare_to_str.item, input_val, normalize)?
                } else {
                    compare_strings(&sim, compare_to_str, normalize, input_val, *input_span)?
                }
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

fn compute_all(s1: &str, s2: &str, norm: bool) -> Result<Value, LabeledError> {
    let span = Span::unknown();
    let algos = vec![
        "bag",
        "cosine",
        "damerau_levenshtein",
        "entropy_ncd",
        "hamming",
        "jaccard",
        "jaro",
        "jaro_winkler",
        "levenshtein",
        "longest_common_subsequence",
        "longest_common_substring",
        "length",
        "lig3",
        "mlipns",
        "overlap",
        "prefix",
        "ratcliff_obershelp",
        "roberts",
        "sift4_common",
        "sift4_simple",
        "smith_waterman",
        "sorensen_dice",
        "suffix",
        "tversky",
        "yujian_bo",
    ];
    let cols = vec!["algorithm".to_string(), "distance".to_string()];
    let mut rows = vec![];
    for algo in algos {
        let sim = Value::string(algo.to_string(), span);
        let val = Value::float(compute(&algo, s1, s2, norm), span);
        rows.push(Value::Record {
            cols: cols.clone(),
            vals: vec![sim, val],
            span,
        });
    }

    Ok(Value::List { vals: rows, span })
}

#[rustfmt::skip]
fn compute(a: &str, s1: &str, s2: &str, norm: bool) -> f64 {
    let sim = a.to_lowercase();
    match sim.as_str() {
        "bag" => if norm { nstr::bag(s1, s2) } else {str::bag(s1, s2) as f64},
        "cos" | "cosine" => if norm { nstr::cosine(s1, s2) } else {str::cosine(s1, s2) as f64},
        "dlev" | "damerau_levenshtein" => if norm { nstr::damerau_levenshtein(s1, s2) } else {str::damerau_levenshtein(s1, s2) as f64},
        "entncd" | "entropy_ncd" => if norm { nstr::entropy_ncd(s1, s2) } else {str::entropy_ncd(s1, s2) as f64},
        "ham" | "hamming" => if norm { nstr::hamming(s1, s2) } else {str::hamming(s1, s2) as f64},
        "jac" | "jaccard" => if norm { nstr::jaccard(s1, s2) } else {str::jaccard(s1, s2) as f64},
        "jar" | "jaro" => if norm { nstr::jaro(s1, s2) } else {str::jaro(s1, s2) as f64},
        "jarw" | "jaro_winkler" => if norm { nstr::jaro_winkler(s1, s2) } else {str::jaro_winkler(s1, s2) as f64},
        "lev" | "levenshtein" => if norm { nstr::levenshtein(s1, s2) } else {str::levenshtein(s1, s2) as f64},
        "lcsubseq" | "longest_common_subsequence" => if norm { nstr::lcsseq(s1, s2) } else {str::lcsseq(s1, s2) as f64},
        "lcsubstr" | "longest_common_substring" => if norm { nstr::lcsstr(s1, s2) } else {str::lcsstr(s1, s2) as f64},
        "len" | "length" => if norm { nstr::length(s1, s2) } else {str::length(s1, s2) as f64},
        "lig" | "lig3" => if norm { nstr::lig3(s1, s2) } else {str::lig3(s1, s2) as f64},
        "mli" | "mlipns" => if norm { nstr::mlipns(s1, s2) } else {str::mlipns(s1, s2) as f64},
        "olap" | "overlap" => if norm { nstr::overlap(s1, s2) } else {str::overlap(s1, s2) as f64},
        "pre" | "prefix" => if norm { nstr::prefix(s1, s2) } else {str::prefix(s1, s2) as f64},
        "rat" | "ratcliff_obershelp" => if norm { nstr::ratcliff_obershelp(s1, s2) } else {str::ratcliff_obershelp(s1, s2) as f64},
        "rob" | "roberts" => if norm { nstr::roberts(s1, s2) } else {str::roberts(s1, s2) as f64},
        "scom" | "sift4_common" => if norm { nstr::sift4_common(s1, s2) } else {str::sift4_common(s1, s2) as f64},
        "ssim" | "sift4_simple" => if norm { nstr::sift4_simple(s1, s2) } else {str::sift4_simple(s1, s2) as f64},
        "smithw" | "smith_waterman" => if norm { nstr::smith_waterman(s1, s2) } else {str::smith_waterman(s1, s2) as f64},
        "soredice" | "sorensen_dice" => if norm { nstr::sorensen_dice(s1, s2) } else {str::sorensen_dice(s1, s2) as f64},
        "suf" | "suffix" => if norm { nstr::suffix(s1, s2) } else {str::suffix(s1, s2) as f64},
        "tv" | "tversky" => if norm { nstr::tversky(s1, s2) } else {str::tversky(s1, s2) as f64},
        "ybo" | "yujian_bo" => if norm { nstr::yujian_bo(s1, s2) } else {str::yujian_bo(s1, s2) as f64},
        _ => if norm { nstr::levenshtein(s1, s2) } else {str::levenshtein(s1, s2) as f64},
    }
}

#[rustfmt::skip]
fn list_algorithms() -> Value {
    let span = Span::unknown();
    let cols = vec!["algorithm".to_string(), "alias".to_string()];
    let mut rows = vec![];

    rows.push(Value::Record {cols: cols.clone(), vals: vec![Value::string("bag", span), Value::string("bag", span)], span});
    rows.push(Value::Record {cols: cols.clone(), vals: vec![Value::string("cosine", span), Value::string("cos", span)], span});
    rows.push(Value::Record {cols: cols.clone(), vals: vec![Value::string("damerau_levenshtein", span), Value::string("dlev", span)], span});
    rows.push(Value::Record {cols: cols.clone(), vals: vec![Value::string("entropy_ncd", span), Value::string("entncd", span)], span});
    rows.push(Value::Record {cols: cols.clone(), vals: vec![Value::string("hamming", span), Value::string("ham", span)], span});
    rows.push(Value::Record {cols: cols.clone(), vals: vec![Value::string("jaccard", span), Value::string("jac", span)], span});
    rows.push(Value::Record {cols: cols.clone(), vals: vec![Value::string("jaro", span), Value::string("jar", span)], span});
    rows.push(Value::Record {cols: cols.clone(), vals: vec![Value::string("jaro_winkler", span), Value::string("jarw", span)], span});
    rows.push(Value::Record {cols: cols.clone(), vals: vec![Value::string("levenshtein", span), Value::string("lev", span)], span});
    rows.push(Value::Record {cols: cols.clone(), vals: vec![Value::string("longest_common_subsequence", span), Value::string("lcsubseq", span)], span});
    rows.push(Value::Record {cols: cols.clone(), vals: vec![Value::string("longest_common_substring", span), Value::string("lcsubstr", span)], span});
    rows.push(Value::Record {cols: cols.clone(), vals: vec![Value::string("length", span), Value::string("len", span)], span});
    rows.push(Value::Record {cols: cols.clone(), vals: vec![Value::string("lig3", span), Value::string("lig", span)], span});
    rows.push(Value::Record {cols: cols.clone(), vals: vec![Value::string("mlipns", span), Value::string("mli", span)], span});
    rows.push(Value::Record {cols: cols.clone(), vals: vec![Value::string("overlap", span), Value::string("olap", span)], span});
    rows.push(Value::Record {cols: cols.clone(), vals: vec![Value::string("prefix", span), Value::string("pre", span)], span});
    rows.push(Value::Record {cols: cols.clone(), vals: vec![Value::string("ratcliff_obershelp", span), Value::string("rat", span)], span});
    rows.push(Value::Record {cols: cols.clone(), vals: vec![Value::string("roberts", span), Value::string("rob", span)], span});
    rows.push(Value::Record {cols: cols.clone(), vals: vec![Value::string("sift4_common", span), Value::string("scom", span)], span});
    rows.push(Value::Record {cols: cols.clone(), vals: vec![Value::string("sift4_simple", span), Value::string("ssim", span)], span});
    rows.push(Value::Record {cols: cols.clone(), vals: vec![Value::string("smith_waterman", span), Value::string("smithw", span)], span});
    rows.push(Value::Record {cols: cols.clone(), vals: vec![Value::string("sorensen_dice", span), Value::string("soredice", span)], span});
    rows.push(Value::Record {cols: cols.clone(), vals: vec![Value::string("suffix", span), Value::string("suf", span)], span});
    rows.push(Value::Record {cols: cols.clone(), vals: vec![Value::string("tversky", span), Value::string("tv", span)], span});
    rows.push(Value::Record {cols: cols.clone(), vals: vec![Value::string("yujian_bo", span), Value::string("ybo", span)], span});

    Value::List {
        vals: rows,
        span,
    }
}

fn compare_strings(
    sim_algo: &str,
    compare_to_str: Spanned<String>,
    normalize: bool,
    input_val: &str,
    input_span: Span,
) -> Result<Value, LabeledError> {
    let compare_from = input_val;
    let compare_to = compare_to_str.item;

    let a_val = compute(sim_algo, compare_from, &compare_to, normalize);

    Ok(Value::Float {
        val: a_val,
        span: input_span,
    })
}

fn main() {
    serve_plugin(&mut StrSimilarity::new(), MsgPackSerializer);
}
