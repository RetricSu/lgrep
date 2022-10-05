use std::cmp;

const NO_OF_CHARS: usize = 256;

pub fn bad_char_heuristic(str: &str, size: i32) -> [i32; NO_OF_CHARS] {
    let init_val: i32 = -1;
    let mut bad_char = [init_val; NO_OF_CHARS];

    for i in 0..size {
        let k = str.as_bytes()[i as usize];
        bad_char[k as usize] = i;
    }
    return bad_char;
}

pub fn search(txt: &str, pattern: &str) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let n = txt.len();
    let m = pattern.len();

    let bad_char = bad_char_heuristic(pattern, m as i32);

    let mut s = 0;

    while s <= (n - m) {
        let mut j = (m - 1) as i32;

        let pat_char = pattern.as_bytes()[j as usize];
        let txt_char = txt.as_bytes()[s + j as usize];

        while j.ge(&0) && pat_char == txt_char {
            j -= 1;
        }

        if j < 0 {
            // push occurrence position into result
            result.push(s as i32);

            /* Shift the pattern so that the next
            character in text aligns with the last
            occurrence of it in pattern.
            The condition s+m < n is necessary for
            the case when pattern occurs at the end
            of text */
            //txt[s+m] is character after the pattern in text
            let dif = if (s + m) < n {
                let pos = txt.as_bytes()[s + m];
                let v = bad_char[pos as usize] as usize;
                let val = m as i32 - v as i32;
                val as i32
            } else {
                1 as i32
            };
            s += dif as usize;
        } else {
            /* Shift the pattern so that the bad character
            in text aligns with the last occurrence of
            it in pattern. The max function is used to
            make sure that we get a positive shift.
            We may get a negative shift if the last
            occurrence  of bad character in pattern
            is on the right side of the current
            character. */

            let pos = txt.as_bytes()[s + j as usize];
            s += cmp::max(1 as i32, j as i32 - bad_char[pos as usize] as i32) as usize;
        };
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::search;

    #[test]
    fn t1() {
        let txt = "ABAAABCDABC";
        let pat = "ABC";
        let v = search(txt, pat);
        assert_eq!(v, [4, 8]);
    }

    #[test]
    fn t2() {
        let txt = "hello, myllo";
        let pat = "llo";
        let v = search(txt, pat);
        assert_eq!(v, [2, 9]);
    }
}
