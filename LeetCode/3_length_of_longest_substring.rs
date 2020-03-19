use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut longest = 0;
        let mut j = 0;
        let mut map = HashMap::new();
        
        for (i, c) in s.char_indices() {
            if let Some(last) = map.insert(c, i + 1) {
                j = j.max(last);
            }
            longest = longest.max(i - j + 1);
        }
        longest as i32
    }
}