use std::collections::HashMap;
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut ht: HashMap<i32, i32> = HashMap::new();
    for (index, num) in nums.iter().enumerate() {
        match ht.get(&(target - num)) {
            Some(v) => return vec![*v, index as i32],
            None => ht.insert(*num, index as i32),
        };
    }
    vec![]
}

#[cfg(test)]
mod tests {
    use super::two_sum;

    #[test]
    fn test_two_sum() {
        let nums = vec![1, 2, 2, 4];
        let target = 5;
        let r = two_sum(nums, target);
        print!(r#"{:?}"#, r)
    }
}
