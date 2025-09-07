pub fn build_proverb(list: &[&str]) -> String {
    if list.len() == 0 {
        return String::new();
    }
    let last = format!("And all for the want of a {}.", list[0].to_string());
    if list.len() == 1 {
        return last;
    }

    let mut proverbs: Vec<String> = vec![];

    for (idx, word) in list.iter().enumerate() {
        if idx < list.len() - 1 {
            let s = format!(
                "For want of a {} the {} was lost.",
                word,
                list[idx + 1].to_string()
            );
            proverbs.push(s);
        }
    }
    proverbs.push(last);
    proverbs.join("\n")
}
