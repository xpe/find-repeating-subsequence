use std::collections::VecDeque;

const DEBUG: bool = false;

/// Returns the factors (>= min) of `x` in increasing order.
fn factors(x: usize, min: usize, max: usize) -> Vec<usize> {
    let mut v = Vec::new();
    for i in min ..= max {
        if x % i == 0 {
            v.push(i);
        }
    }
    v
}

/// Returns an initialized table for use with `detect_pattern`.
fn new_table(min: usize, max: usize) -> Vec<Option<usize>> {
    let mut t = Vec::with_capacity(max - min + 1);
    for _i in min ..= max {
        t.push(Some(0));
    }
    t
}

fn max_len(table: &[Option<usize>], min: usize, max: usize) -> Option<usize> {
    for i in (min ..= max).rev() {
        if table[i - min].is_some() {
            return Some(i);
        }
    }
    None
}

/// Determine if there is a repeating pattern in a vector of items.
///
/// The definition of 'pattern' used by this function is this:
/// the pattern starts at the beginning of the sequence and repeats
/// until the sequence ends.
///
/// Returns the pattern length, if found. Otherwise, returns `None`.
///
/// Example:
///
/// s = vec![
///   0, 0, 1, 0, 1,
///   0, 0, 1, 0, 1,
///   0, 0, 1, 0, 1,
///   0, 0, 1, 0, 1,
///   0, 0, 1, 0, 1,
///   0, 0, 1, 0, 1,
///   0, 0, 1, 0, 1,
///   0, 0, 1, 0, 1,
///   0, 0, 1, 0, 1,
///   0, 0, 1, 0, 1,
/// ]
/// a (minimum pattern length) = 2
/// b (maximum pattern length) = 20
///
/// Implementation details:
/// * table of work done so far
/// * queue of lengths to check next
fn detect_pattern<T: PartialEq>(
    s: Vec<T>, min: usize, max: usize
) -> Option<usize> {
    let mut table = new_table(min, max);
    let mut queue: VecDeque<usize> = VecDeque::new();

    // Phase 1 Loop
    while let Some(len) = max_len(&table, min, max) {
        if DEBUG { println!("P1"); }
        let idx = len - min;
        let x = table[idx].unwrap(); // `max_len` guarantees some value
        if x < len {
            let mut y = x + len;
            if y >= s.len() {
                // A pattern is expected to have at least one subsequent copy (but does not).
                // Therefore, rule out patterns of length `len`.
                if DEBUG { println!("  len:{} x:{} y:{} y >= s.len()", len, x, y); }
                table[idx] = None;
            } else {
                let mut matching = true;
                while matching && y < s.len() {
                    matching = s[x] == s[y];
                    y += len;
                }
                if DEBUG { println!("  len:{} x:{} matching:{}", len, x, matching); }
                if matching {
                    // Advance progress in confirming current pattern search.
                    table[idx] = Some(x + 1);
                } else {
                    // Rule out patterns of length (`len` and factors of `len`).
                    for factor in factors(len, min, len) {
                        table[factor - min] = None;
                    }
                }
            }
        } else {
            if DEBUG { println!("  pattern found"); }
            // We found a pattern of length `len`.
            // Next, check pattern lengths that are factors of `len`.
            for factor in factors(len, min, len - 1) {
                queue.push_front(factor);
            }
            break;
        }
    }

    // Optional Phase 2 Loop
    while let Some(&len) = queue.get(0) {
        if DEBUG { println!("P2 : queue:{:?}", queue); }
        let idx = len - min;
        match table[idx] {
            None => {
                queue.pop_front();
            },
            Some(x) => {
                if x < len {
                    let mut y = x + len;
                    if y >= s.len() {
                        // A pattern is expected to have at least one subsequent copy (but does not).
                        // Therefore, rule out patterns of length `len`.
                        if DEBUG { println!("  len:{} x:{} y:{} y >= s.len()", len, x, y); }
                        table[idx] = None;
                        queue.pop_front();
                    } else {
                        let mut matching = true;
                        while matching && y < s.len() {
                            matching = s[x] == s[y];
                            y += len;
                        }
                        if DEBUG { println!("  len:{} x:{} matching:{}", len, x, matching); }
                        if matching {
                            // Advance progress in confirming current pattern search.
                            table[idx] = Some(x + 1);
                        } else {
                            // Rule out patterns of length `len`.
                            table[idx] = None;
                            queue.pop_front();
                        }
                    }
                } else {
                    if DEBUG { println!("    pattern found"); }
                    // clear and add new factors to queue
                    queue.clear();
                    for factor in factors(len, min, len - 1) {
                        queue.push_front(factor);
                    }
                }
            },
        }
    }

    // Lookup result
    for i in min ..= max {
        if let Some(x) = table[i - min] {
            if x == i { return Some(x); }
        }
    }
    None
}

pub fn test_factors() {
    let factors = factors(24, 3, 23);
    println!("{:?}", factors);
}

/// Returns a vector consisting of `n` copies of `pattern`.
pub fn build_vec<T: Clone>(pattern: Vec<T>, n: usize) -> Vec<T> {
    let mut v = Vec::with_capacity(pattern.len() * n);
    for _ in 0 .. n {
        v.extend(pattern.clone());
    }
    v
}


pub fn test_detect_pattern_1() {
    let p = vec![0, 0, 1, 0, 1];
    let v = build_vec(p, 30);
    println!("{:?}", detect_pattern(v, 3, 40));
}

pub fn test_detect_pattern_2() {
    let p = vec![0, 0, 1, 0, 1, 2];
    let v = build_vec(p, 20);
    println!("{:?}", detect_pattern(v, 3, 80));
}

pub fn test_detect_pattern_3() {
    let p = vec![0, 0, 1, 0, 1, 2, 1, 1, 1, 1];
    let v = build_vec(p, 30);
    println!("{:?}", detect_pattern(v, 3, 100));
}

pub fn test_detect_pattern_4() {
    let p = vec![0, 0, 1, 0, 1, 2, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 2];
    let v = build_vec(p, 20);
    println!("{:?}", detect_pattern(v, 3, 500));
}

pub fn test_detect_pattern_5() {
    let p = vec![
        0, 0, 1, 0, 1, 2, 1, 1, 1, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0,
        0, 0, 1, 0, 1, 2, 1, 1, 1, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1,
        0, 0, 1, 0, 1, 2, 1, 1, 1, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 2,
        0, 0, 1, 0, 1, 2, 1, 1, 1, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 3,
    ];
    let v = build_vec(p, 100);
    println!("{:?}", detect_pattern(v, 3, 2000));
}

fn main() {
    test_factors();
    test_detect_pattern_1();
    test_detect_pattern_2();
    test_detect_pattern_3();
    test_detect_pattern_4();
    test_detect_pattern_5();
}
