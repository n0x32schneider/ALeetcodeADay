impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        // use two indices l and r to signal start and end of the current substring
        // store the most recent index of each character found in a hashmap (char) -> (index).
        //
        // Simple algorithm: Iterate once over the string from left to right (-> O (n)).
        // For each new character, check if it is in the hashmap. If it is, and the index c_i is bigger than the start of the current substring,
        // check if the current substring length is bigger than the maximum length and save the result. Furthermore
        // reset the l index of the substring to c_i + 1, and calculate the current substring length. Then continue the search.
        // In case index c_i is not in the current substring, update c_i and continue the search without resetting the substring.

        let mut l = 0;
        let mut r = 0;

        let mut current_ss_len = 0;
        let mut max_ss_len : i32 = 0;

        // edge cases:
        if s.len() < 2{
           return s.len() as i32; 
        }

        // Max number of characters is limited through char size (1 byte), if we do assume standard ASCII encoding
        let mut found_chars_hm = std::collections::HashMap::with_capacity(255);

        for (ind, val) in s.chars().enumerate() {
            // new char is in substring, update l index
            if let Some(&v) = found_chars_hm.get(&val) {
                if v >= l {
                    l = v + 1;
                } 
            }
            // insert up to date character index in hashmap
            found_chars_hm.insert(val, ind as i32);
            // update r to current index
            r = ind as i32;
            // calculate substring length and update values
            current_ss_len = r -l + 1;
            if current_ss_len > max_ss_len {
                max_ss_len = current_ss_len;
            }
        }
        return max_ss_len;

    }
}