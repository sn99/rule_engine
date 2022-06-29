# rule_engine

Eg:

```rust
use rule_engine;
use std::collections::BTreeMap;

fn main() {
    let tree = rule_engine::and(vec![
        rule_engine::string_equals("Name is John Doe", "name", "John Doe"),
        rule_engine::or(vec![
            rule_engine::int_equals("Favorite number is 10", "fav_number", 10),
            rule_engine::int_range(
                "Fav number between 11 and 16",
                "fav_number",
                11,
                16,
            ),
        ]),
    ]);
    let mut facts = BTreeMap::new();
    facts.insert("name".into(), "John Doe".into());
    facts.insert("fav_number".into(), "11".into());
    let result = tree.check(&facts);
    println!("{:#?}", result);

    assert_eq!(result.status, rule_engine::Status::Met);
}
```

This should run without panicking giving the following output:

```rust
RuleResult {
    name: "And",
    status: Met,
    children: [
        RuleResult {
            name: "Name is John Doe",
            status: Met,
            children: [],
        },
        RuleResult {
            name: "Or",
            status: Met,
            children: [
                RuleResult {
                    name: "Favorite number is 10",
                    status: NotMet,
                    children: [],
                },
                RuleResult {
                    name: "Fav number between 11 and 16",
                    status: Met,
                    children: [],
                },
            ],
        },
    ],
}

```