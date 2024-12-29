use std::collections::HashSet;

fn main() {
    let input = "abcaaaaa";

    // Call your function here
    let result = length_of_longest_substring(input);

    // Print the result
    println!("The length of the longest substring without repeating characters is: {}", result);
}

fn length_of_longest_substring(s: &str) -> usize {
    let mut hash_set: HashSet<char> = HashSet::new();
    let mut longest_len: usize = 0;
    let mut start_pos: usize = 0;

    // Your logic here
    for (current_idx, one_char) in s.chars().enumerate()
    {
        if hash_set.contains(&one_char)
        {
            start_pos += 1;
        } else
        {
            hash_set.insert(one_char);
        }

        longest_len = longest_len.max(current_idx - start_pos + 1);
    }

    longest_len
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_normal_input() {
        let input = "abcabcabc";

        let result = super::length_of_longest_substring(input);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_case_1() {
        let input = "abcbedfgc";

        let result = super::length_of_longest_substring(input);
        assert_eq!(result, 7);
    }

    #[test]
    fn test_null_str() {
        let input = "";

        let result = super::length_of_longest_substring(input);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_one_char() {
        let input = "a";

        let result = super::length_of_longest_substring(input);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_repeat() {
        let input = "aaaaaaaaaaaaaa";

        let result = super::length_of_longest_substring(input);
        assert_eq!(result, 1);
    }
}