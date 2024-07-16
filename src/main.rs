use regex::bytes::RegexBuilder;

const PATTERN: &str = r#"(?:\bgs://([0-9a-zA-Z][0-9a-zA-Z._-]{1,61}[0-9a-zA-Z])\b)|(?:([0-9a-zA-Z][0-9a-zA-Z._-]{1,61}[0-9a-zA-Z])\.storage\.googleapis\.com)|(?:\bstorage\.googleapis\.com/([0-9a-zA-Z][0-9a-zA-Z._-]{1,61}[0-9a-zA-Z]))"#;

fn main() {
    // Crashes with unicode disabled + case insensitive + large dfa size limit
    // unicode disabled + case insensitive + 869_448_557 dfa size limit works though
    let re = RegexBuilder::new(PATTERN)
        .unicode(false)
        .case_insensitive(true)
        //.dfa_size_limit(869_448_557) // Works
        .dfa_size_limit(869_448_558) // Does not work
        .build()
        .unwrap();

    let file = std::fs::read("haystack").unwrap();

    for m in re.find_iter(&file) {
        println!("{m:?}");
    }

}
