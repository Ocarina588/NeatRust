impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();

        for i in 0..nums.len() {
            if let Some(value) = map.get(&(target - nums[i])) {
                return vec![i as i32, *value];
            }
            map.insert(nums[i], i as i32);
        }
        return vec![-1, -1];
    }
}