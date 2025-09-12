use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut ans = BTreeMap::new();

    for (&key, value) in h {
        for v in value {
            ans.insert(v.to_ascii_lowercase(), key);
        }
    }
    ans
}
