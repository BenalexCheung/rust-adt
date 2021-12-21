struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut i = 0;
        let mut j = 0;
        while j < nums.len() {
            if nums[j] != 0 {
                nums[i] = nums[j];
                i += 1;
            }
            j += 1;
        }
        let mut k = i;
        while k < nums.len() {
            nums[k] = 0;
            k += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut input = vec![0, 1, 0, 3, 12];
        println!("input: {:?}", input);
        Solution::move_zeroes(&mut input);
        println!("output: {:?}", input);
        assert_eq!(input, vec![1, 3, 12, 0, 0]);
    }
}
