#![allow(unused)]

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn group_anagram(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut vecs: Vec<Vec<String>> = Vec::new();
        let mut map: HashMap<String, Vec<String>> = HashMap::new();

        for i in 0..strs.len() {
            let mut chars = vec![];
            for c in strs[i].chars() {
                chars.push(c);
            }
            chars.sort();

            let key: String = chars.into_iter().collect();

            let value = map.get(&key);
            if value != None {
                let mut v = value.unwrap().to_vec();
                v.push(strs[i].clone());
                map.insert(key, v);
            } else {
                let v = vec![strs[i].clone()];
                map.insert(key, v);
            }
        }

        for val in map.values() {
            vecs.push(val.to_vec());
        }
        vecs
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_group_anagram() {
        let strs = vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ];
        let v = Solution::group_anagram(strs);
        assert_eq!(v.len(), 3);
    }
}
