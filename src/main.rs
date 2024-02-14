use std::vec;

use nu_plugin::{serve_plugin, EvaluatedCall, LabeledError, MsgPackSerializer, Plugin};
use nu_protocol::{
    record, Category, PluginExample, PluginSignature, Span, Spanned, SyntaxShape, Value,
};
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
            .plugin_examples(vec![
                PluginExample {
                    description: "Compare two strings for similarity".into(),
                    example: "'nutshell' | str similarity 'nushell'".into(),
                    result: None,
                },
                PluginExample {
                    description:
                        "Compare two strings for similarity and normalize the output value".into(),
                    example: "'nutshell' | str similarity -n 'nushell'".into(),
                    result: None,
                },
                PluginExample {
                    description: "Compare two strings for similarity using a specific algorithm"
                        .into(),
                    example: "'nutshell' | str similarity 'nushell' -a levenshtein".into(),
                    result: None,
                },
                PluginExample {
                    description: "List all the included similarity algorithms".into(),
                    example: "str similarity 'nu' --list".into(),
                    result: None,
                },
                PluginExample {
                    description: "Compare two strings for similarity with all algorithms".into(),
                    example: "'nutshell' | str similarity 'nushell' -A".into(),
                    result: None,
                },
                PluginExample {
                    description: "Compare two strings for similarity with all algorithms and normalize the output value".into(),
                    example: "'nutshell' | str similarity 'nushell' -A -n".into(),
                    result: None,
                },
            ])]
    }

    fn run(
        &mut self,
        name: &str,
        _config: &Option<Value>,
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
        let normalize = call.has_flag("normalize")?;
        let list = call.has_flag("list")?;
        if list {
            return Ok(list_algorithms());
        }
        let algo: Option<String> = call.get_flag("algorithm")?;
        let sim = match algo {
            Some(a) => a.to_string(),
            None => "levenshtein".to_string(),
        };
        let all = call.has_flag("all")?;
        let input_span = input.span();

        let ret_val = match input {
            Value::String { val: input_val, .. } => {
                if all {
                    compute_all(&compare_to_str.item, input_val, normalize)?
                } else {
                    compare_strings(&sim, compare_to_str, normalize, input_val, input_span)?
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
    let mut rows = vec![];
    for algo in algos {
        let sim = Value::string(algo.to_string(), span);
        let val_comp = compute(&algo, s1, s2, norm);
        let val = if val_comp.fract() == 0.0 {
            Value::int(val_comp as i64, span)
        } else {
            Value::float(val_comp, span)
        };
        rows.push(Value::test_record(
            record! { "algorithm" => sim, "distance" => val },
        ));
    }

    Ok(Value::test_list(rows))
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
    let mut rows = vec![];

    rows.push(Value::test_record(record! { "algorithm" => Value::test_string("bag"), "short" => Value::test_string("bag")}));
    rows.push(Value::test_record(record! { "algorithm" => Value::test_string("cosine"), "short" => Value::test_string("cos")}));
    rows.push(Value::test_record(record! { "algorithm" => Value::test_string("damerau_levenshtein"), "short" => Value::test_string("dlev")}));
    rows.push(Value::test_record(record! { "algorithm" => Value::test_string("entropy_ncd"), "short" => Value::test_string("entncd")}));
    rows.push(Value::test_record(record! { "algorithm" => Value::test_string("hamming"), "short" => Value::test_string("ham")}));
    rows.push(Value::test_record(record! { "algorithm" => Value::test_string("jaccard"), "short" => Value::test_string("jac")}));
    rows.push(Value::test_record(record! { "algorithm" => Value::test_string("jaro"), "short" => Value::test_string("jar")}));
    rows.push(Value::test_record(record! { "algorithm" => Value::test_string("jaro_winkler"), "short" => Value::test_string("jarw")}));
    rows.push(Value::test_record(record! { "algorithm" => Value::test_string("levenshtein"), "short" => Value::test_string("lev")}));
    rows.push(Value::test_record(record! { "algorithm" => Value::test_string("longest_common_subsequence"), "short" => Value::test_string("lcsubseq")}));
    rows.push(Value::test_record(record! { "algorithm" => Value::test_string("longest_common_substring"), "short" => Value::test_string("lcsubstr")}));
    rows.push(Value::test_record(record! { "algorithm" => Value::test_string("length"), "short" => Value::test_string("len")}));
    rows.push(Value::test_record(record! { "algorithm" => Value::test_string("lig3"), "short" => Value::test_string("lig")}));
    rows.push(Value::test_record(record! { "algorithm" => Value::test_string("mlipns"), "short" => Value::test_string("mli")}));
    rows.push(Value::test_record(record! { "algorithm" => Value::test_string("overlap"), "short" => Value::test_string("olap")}));
    rows.push(Value::test_record(record! { "algorithm" => Value::test_string("prefix"), "short" => Value::test_string("pre")}));
    rows.push(Value::test_record(record! { "algorithm" => Value::test_string("ratcliff_obershelp"), "short" => Value::test_string("rat")}));
    rows.push(Value::test_record(record! { "algorithm" => Value::test_string("roberts"), "short" => Value::test_string("rob")}));
    rows.push(Value::test_record(record! { "algorithm" => Value::test_string("sift4_common"), "short" => Value::test_string("scom")}));
    rows.push(Value::test_record(record! { "algorithm" => Value::test_string("sift4_simple"), "short" => Value::test_string("ssim")}));
    rows.push(Value::test_record(record! { "algorithm" => Value::test_string("smith_waterman"), "short" => Value::test_string("smithw")}));
    rows.push(Value::test_record(record! { "algorithm" => Value::test_string("sorensen_dice"), "short" => Value::test_string("soredice")}));
    rows.push(Value::test_record(record! { "algorithm" => Value::test_string("suffix"), "short" => Value::test_string("suf")}));
    rows.push(Value::test_record(record! { "algorithm" => Value::test_string("tversky"), "short" => Value::test_string("tv")}));
    rows.push(Value::test_record(record! { "algorithm" => Value::test_string("yujian_bo"), "short" => Value::test_string("ybo")}));

    Value::test_list(rows)
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

    if a_val.fract() == 0.0 {
        Ok(Value::int(a_val as i64, input_span))
    } else {
        Ok(Value::float(a_val, input_span))
    }
}

fn main() {
    serve_plugin(&mut StrSimilarity::new(), MsgPackSerializer);
}
