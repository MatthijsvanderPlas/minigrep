use minigrep::lib::search;
use minigrep::lib::search_case_insensitive;

#[test]
fn one_result() {
    let query = "duct";
    let contents = "\
Rust:

safe, fast, productive.
Pick three.";
    let results = search(query, contents);
    assert_eq!(vec!["safe, fast, productive."], results);
}

#[test]
fn case_sensitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
    assert_eq!(
        vec!["Rust:", "Trust me."],
        search_case_insensitive(query, contents)
    );
}
