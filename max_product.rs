use std::collections::HashMap;


pub fn max_product(nums: Vec<i32>) -> i32 {
    let mut hm: HashMap<i32,i32> = HashMap::new();
    hm.insert(1,0);
    hm.insert(2, 0);
    for (_, v) in nums.iter().enumerate() {
        let max =hm.get::<i32>(&1);
        let min = hm.get(&2);
        if v > max.unwrap(){
            hm.insert(2, hm[&1]);
            hm.insert(1, *v);
        }else if v > min.unwrap(){
            hm.insert(1, *v);
        }
    }
    let result = hm[&1] * hm[&2];
    return result
}



#[cfg(test)]
mod tests {
    use super::max_product;

    #[test]
    fn test_max_product() {
        let r= max_product(vec![3,4,5,2]);
        println!("{:?}", r)
    }
}
