fn main() {
    valid_palindrome("abca".to_string());
}

fn helper(s: &[u8], mut left: usize, mut right: usize) -> bool {
    while left < right {
        if s[left] != s[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }
    true
}

pub fn valid_palindrome(s: String) -> bool {
    let s = s.as_bytes();
    let mut l = 0;
    let mut r = s.len() - 1 as usize;
    while l < r {
        if s[l] != s[r] {
            return helper(s, l, r - 1) || helper(s, l + 1, r);
        }
        l += 1;
        r -= 1;
    }
    true
}
