use std::collections::BTreeMap;

fn main() {
    let tree = rule_engine::and(vec![
        rule_engine::string_equals("Name is John Doe", "name", "John Doe"),
        rule_engine::or(vec![
            rule_engine::int_equals("Favorite number is 10", "fav_number", 10),
            rule_engine::int_range("Fav number between 11 and 16", "fav_number", 11, 16),
        ]),
    ]);
    let mut facts = BTreeMap::new();
    facts.insert("name".into(), "John Doe".into());
    facts.insert("fav_number".into(), "10".into());
    let result = tree.check(&facts);
    println!("{:#?}", result);
    assert_eq!(result.status, rule_engine::Status::Met);
}
