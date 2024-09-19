I'm archiving this repo because I combined this functionality in the `nu_plugin_strutils` nushell plugin at https://github.com/fdncred/nu_plugin_strutils

# nu_plugin_str_similarity

This plugin uses the [textdistance.rs](https://crates.io/crates/textdistance) crate to calculate edit distance

## Usage:

### Single Algorithm Usage

```shell
❯ "nushell" | str similarity "nutshell" --algorithm levenshtein
1
```

### All Algorithms Usage

```bash
❯ "nushell" | str similarity "nutshell" -all
```
```
╭────┬────────────────────────────┬──────────╮
│  # │         algorithm          │ distance │
├────┼────────────────────────────┼──────────┤
│  0 │ bag                        │        1 │
│  1 │ cosine                     │     0.94 │
│  2 │ damerau_levenshtein        │        1 │
│  3 │ entropy_ncd                │     0.05 │
│  4 │ hamming                    │        5 │
│  5 │ jaccard                    │     0.88 │
│  6 │ jaro                       │     0.96 │
│  7 │ jaro_winkler               │     0.97 │
│  8 │ levenshtein                │        1 │
│  9 │ longest_common_subsequence │        7 │
│ 10 │ longest_common_substring   │        5 │
│ 11 │ length                     │        1 │
│ 12 │ lig3                       │     0.86 │
│ 13 │ mlipns                     │        0 │
│ 14 │ overlap                    │        1 │
│ 15 │ prefix                     │        2 │
│ 16 │ ratcliff_obershelp         │     0.93 │
│ 17 │ roberts                    │     0.93 │
│ 18 │ sift4_common               │        1 │
│ 19 │ sift4_simple               │        1 │
│ 20 │ smith_waterman             │        6 │
│ 21 │ sorensen_dice              │     0.93 │
│ 22 │ suffix                     │        5 │
│ 23 │ tversky                    │     0.88 │
│ 24 │ yujian_bo                  │     0.12 │
├────┼────────────────────────────┼──────────┤
│  # │         algorithm          │ distance │
╰────┴────────────────────────────┴──────────╯
```

### All Algorithms Normalized Usage

The output is normalized between 0 and 1

```shell
❯ "nushell" | str similarity "nutshell" --all --normalize
```
```
╭────┬────────────────────────────┬──────────╮
│  # │         algorithm          │ distance │
├────┼────────────────────────────┼──────────┤
│  0 │ bag                        │     0.12 │
│  1 │ cosine                     │     0.94 │
│  2 │ damerau_levenshtein        │     0.12 │
│  3 │ entropy_ncd                │     0.05 │
│  4 │ hamming                    │     0.62 │
│  5 │ jaccard                    │     0.88 │
│  6 │ jaro                       │     0.96 │
│  7 │ jaro_winkler               │     0.97 │
│  8 │ levenshtein                │     0.12 │
│  9 │ longest_common_subsequence │     0.88 │
│ 10 │ longest_common_substring   │     0.62 │
│ 11 │ length                     │     0.12 │
│ 12 │ lig3                       │     0.86 │
│ 13 │ mlipns                     │        0 │
│ 14 │ overlap                    │        1 │
│ 15 │ prefix                     │     0.25 │
│ 16 │ ratcliff_obershelp         │     0.93 │
│ 17 │ roberts                    │     0.93 │
│ 18 │ sift4_common               │     0.12 │
│ 19 │ sift4_simple               │     0.12 │
│ 20 │ smith_waterman             │     0.75 │
│ 21 │ sorensen_dice              │     0.93 │
│ 22 │ suffix                     │     0.62 │
│ 23 │ tversky                    │     0.88 │
│ 24 │ yujian_bo                  │     0.12 │
├────┼────────────────────────────┼──────────┤
│  # │         algorithm          │ distance │
╰────┴────────────────────────────┴──────────╯
```

## List the available algorithms and aliases

```
❯ "nushell" | str similarity "nutshell" --list
╭────┬────────────────────────────┬──────────╮
│  # │         algorithm          │  alias   │
├────┼────────────────────────────┼──────────┤
│  0 │ bag                        │ bag      │
│  1 │ cosine                     │ cos      │
│  2 │ damerau_levenshtein        │ dlev     │
│  3 │ entropy_ncd                │ entncd   │
│  4 │ hamming                    │ ham      │
│  5 │ jaccard                    │ jac      │
│  6 │ jaro                       │ jar      │
│  7 │ jaro_winkler               │ jarw     │
│  8 │ levenshtein                │ lev      │
│  9 │ longest_common_subsequence │ lcsubseq │
│ 10 │ longest_common_substring   │ lcsubstr │
│ 11 │ length                     │ len      │
│ 12 │ lig3                       │ lig      │
│ 13 │ mlipns                     │ mli      │
│ 14 │ overlap                    │ olap     │
│ 15 │ prefix                     │ pre      │
│ 16 │ ratcliff_obershelp         │ rat      │
│ 17 │ roberts                    │ rob      │
│ 18 │ sift4_common               │ scom     │
│ 19 │ sift4_simple               │ ssim     │
│ 20 │ smith_waterman             │ smithw   │
│ 21 │ sorensen_dice              │ soredice │
│ 22 │ suffix                     │ suf      │
│ 23 │ tversky                    │ tv       │
│ 24 │ yujian_bo                  │ ybo      │
├────┼────────────────────────────┼──────────┤
│  # │         algorithm          │  alias   │
╰────┴────────────────────────────┴──────────╯
```
