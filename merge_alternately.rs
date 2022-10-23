pub fn merge_alternately(word1: String, word2: String) -> String {
    let mut result = String::new();
    for (index, value) in word1.as_bytes().iter().enumerate() {
        if index >= word2.len() {
            result.push_str(&word1.as_str()[index..word1.len()]);
            break;
        } else if index == word1.len() - 1 {
            result.push(*value as char);
            result.push_str(&word2.as_str()[index..]);
            break;
        }
        result.push(*value as char);
        result.push({
            let this = word2.chars().nth(index);
            match this {
                Some(x) => x,
                None => Default::default(),
            }
        })
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::merge_alternately;

    #[test]
    fn test_merge_alternately() {
        let r: String = merge_alternately(String::from("abcd"), String::from("pq"));
        println!("{:?}", r)
    }
}
