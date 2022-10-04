use std::cmp;

const NO_OF_CHARS: usize = 256;

pub fn bad_char_heuristic(str: String, size: i32) -> [i32; NO_OF_CHARS] {
    let init_val: i32 = -1;
    let mut bad_char = [init_val; NO_OF_CHARS];

    for i in 0..size {
        let k = str.as_bytes()[i as usize];
        println!("k: {}, size: {}, i: {}", k, size, i);
        bad_char[k as usize] = i;
    }
    return bad_char;
}

pub fn search(txt: &str, pattern: &str) {
    let n = txt.len();
    let m = pattern.len();
    let bad_char = bad_char_heuristic(pattern.to_string(), m as i32);

    let mut s = 0;

    while s <= (n - m) {
        let mut j = (m - 1) as i32;

        let pat_char = pattern.as_bytes()[j as usize];
        let txt_char = txt.as_bytes()[s + j as usize];

        while j.ge(&0) && pat_char == txt_char {
            j -= 1;
        }

        if j < 0 {
            println!("Patterns occur at shift = {}", s);

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
            println!("j: {}, {}", j, bad_char[pos as usize] as usize);
            s += cmp::max(1, j as usize - bad_char[pos as usize] as usize);
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::search;

    #[test]
    fn test() {
        let txt = "ABAAABCDABC";
        let pat = "ABC";
        search(txt, pat);
    }
}
