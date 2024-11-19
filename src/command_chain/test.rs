#[cfg(test)]
mod tests {
    use crate::command_chain::map::*;

    use expect_test::{expect, Expect};
    use im::Vector;

    fn to_string(map: &Map<i32>) -> String {
        fn rec(output: &mut Vec<String>, key: &mut String, map: &Node<i32>) {
            match map {
                Node::Value(i) => output.push(format!("{}: {}", key, i)),
                Node::SubTree(ord_map) => {
                    ord_map.iter().for_each(|(c, node)| {
                        key.push(*c);
                        rec(output, key, node);
                        key.pop();
                    });
                }
            }
        }

        let mut output = Vec::new();
        let mut key = String::new();
        match map {
            Map::Empty => (),
            Map::NonEmpty(node) => rec(&mut output, &mut key, node),
        }
        output.join("\n")
    }

    fn expect_test(map: Result<Map<i32>, String>, expect: Expect) {
        let actual = match map {
            Ok(map) => to_string(&map),
            Err(err) => err,
        };
        expect.assert_eq(&actual.to_string());
    }

    #[test]
    fn basic() {
        let map = (|| {
            let map = Map::new();
            let map = map.with("abc".to_owned(), 1)?;
            let map = map.with("abd".to_owned(), 2)?;
            let map = map.with("abe".to_owned(), 3)?;
            let map = map.with("cast".to_owned(), 4)?;
            let map = map.with("case".to_owned(), 5)?;
            let map = map.with("cass".to_owned(), 6)?;
            Ok(map)
        })();
        expect_test(
            map,
            expect![[r#"
                abc: 1
                abd: 2
                abe: 3
                case: 5
                cass: 6
                cast: 4"#]],
        );
    }

    #[test]
    fn add_superset_err() {
        let map = (|| {
            let map = Map::new();
            let map = map.with("abc".to_owned(), 1)?;
            let map = map.with("abcd".to_owned(), 2)?;
            Ok(map)
        })();
        expect_test(
            map,
            expect!["Attempting to add a command chain whose prefix already exists in the mapping"],
        );
    }

    #[test]
    fn add_prefix_err() {
        let map = (|| {
            let map = Map::new();
            let map = map.with("abc".to_owned(), 1)?;
            let map = map.with("ab".to_owned(), 2)?;
            Ok(map)
        })();
        expect_test(
            map,
            expect![
                "Attempting to add a prefix of a command chain which already exists in the mapping"
            ],
        );
    }

    fn ord_map_empty_checks(map: &Map<i32>) -> Result<(), &'static str> {
        fn rec(node: &Node<i32>) -> Result<(), &'static str> {
            match node {
                Node::SubTree(ord_map) => match ord_map.is_empty() {
                    true => Err("Non-empty sub map invariant violated"),
                    false => Ok(()),
                },
                Node::Value(_) => Ok(()),
            }
        }
        match map {
            Map::Empty => Ok(()),
            Map::NonEmpty(node) => rec(node),
        }
    }

    #[test]
    fn with_removals() {
        let map = (|| {
            let map = Map::new();
            let map = map.with("abc".to_owned(), 1)?;
            let map = map.with("abd".to_owned(), 2)?;
            let map = map.with("abe".to_owned(), 3)?;
            let map = map.with("cast".to_owned(), 4)?;
            let map = map.with("case".to_owned(), 5)?;
            let map = map.with("cass".to_owned(), 6)?;
            let map = map.without("ca".to_owned());
            ord_map_empty_checks(&map)?;
            let map = map.without("a".to_owned());
            ord_map_empty_checks(&map)?;
            let map = map.without("".to_owned());
            ord_map_empty_checks(&map)?;
            let map = map.without("abc".to_owned());
            ord_map_empty_checks(&map)?;
            let map = map.without("abe".to_owned());
            ord_map_empty_checks(&map)?;
            let map = map.without("cast".to_owned());
            ord_map_empty_checks(&map)?;
            Ok(map)
        })();
        expect_test(
            map,
            expect![[r#"
                abd: 2
                case: 5
                cass: 6"#]],
        );
    }

    fn key_to_vec(key: String) -> Vector<char> {
        let mut chars = Vector::new();
        key.chars().into_iter().for_each(|c| {
            chars.push_back(c);
        });
        chars
    }

    fn search_result_expect_test(
        search_result: Result<SearchResult<i32>, &'static str>,
        expect: Expect,
    ) {
        let actual = match search_result {
            Err(err) => err.to_owned(),
            Ok(SearchResult::None) => "None".to_owned(),
            Ok(SearchResult::Value(i)) => format!("Value: {}", i),
            Ok(SearchResult::Children(children)) => {
                let children: String = children.iter().collect();
                format!("Children: {}", children)
            }
        };
        expect.assert_eq(&actual.to_string());
    }

    #[test]
    fn search_value() {
        let search_result = (|| {
            let map = Map::new();
            let map = map.with("abc".to_owned(), 1)?;
            let map = map.with("abd".to_owned(), 2)?;
            let map = map.with("abe".to_owned(), 3)?;
            let map = map.with("cast".to_owned(), 4)?;
            let map = map.with("case".to_owned(), 5)?;
            let map = map.with("cass".to_owned(), 6)?;
            let map = map.without("ca".to_owned());
            let map = map.without("a".to_owned());
            let map = map.without("".to_owned());
            let map = map.without("abc".to_owned());
            let map = map.without("abe".to_owned());
            let map = map.without("cast".to_owned());
            Ok(map.search(key_to_vec("case".to_owned())))
        })();

        search_result_expect_test(search_result, expect!["Value: 5"]);
    }

    #[test]
    fn search_children() {
        let search_result = (|| {
            let map = Map::new();
            let map = map.with("abc".to_owned(), 1)?;
            let map = map.with("abd".to_owned(), 2)?;
            let map = map.with("cast".to_owned(), 4)?;
            let map = map.with("case".to_owned(), 5)?;
            let map = map.with("cass".to_owned(), 6)?;
            Ok(map.search(key_to_vec("ab".to_owned())))
        })();

        search_result_expect_test(search_result, expect!["Children: cd"]);
    }

    #[test]
    fn search_children_deep() {
        let search_result = (|| {
            let map = Map::new();
            let map = map.with("abc".to_owned(), 1)?;
            let map = map.with("abd".to_owned(), 2)?;
            let map = map.with("abef".to_owned(), 2)?;
            let map = map.with("abecadaba".to_owned(), 2)?;
            let map = map.with("abridge".to_owned(), 2)?;
            Ok(map.search(key_to_vec("ab".to_owned())))
        })();

        search_result_expect_test(search_result, expect!["Children: cder"]);
    }

    #[test]
    fn search_none() {
        let search_result = (|| {
            let map = Map::new();
            let map = map.with("abc".to_owned(), 1)?;
            let map = map.with("abd".to_owned(), 2)?;
            Ok(map.search(key_to_vec("abce".to_owned())))
        })();

        search_result_expect_test(search_result, expect!["None"]);
    }
}
