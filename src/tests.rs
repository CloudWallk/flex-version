use super::*;

fn parse(input: &str) -> Version {
    input.parse().unwrap()
}

#[test]
fn test_equals() {
    let expected: Version = parse("1.2");

    assert_eq!(parse("1.2.0"), expected);
    assert_eq!(parse("1.2.0.0.0.0"), expected);
    assert_ne!(parse("1.2.0.0.0.1"), expected);
    assert_ne!(parse("1.2.0.0.0.a"), expected);
}

#[test]
fn test_ordering() {
    let versions: Vec<Version> = vec![
        parse("0.9"),
        parse("1.0.a.2"),
        parse("1.0.b1"),
        parse("1.0"),
        parse("1"),
        parse("1.0.0.0"),
        parse("1.0.1"),
    ];

    let mut versions_sorted = versions.clone();
    versions_sorted.reverse();
    versions_sorted.sort();

    assert_eq!(versions, versions_sorted)
}
