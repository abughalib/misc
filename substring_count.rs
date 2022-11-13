/* Question

**Substring count**

You are given a string S of length N consisting of lowercase English alphabets and an integer K.

You need to partition the string until it is empty in the following way,

1. Select a maximum length substring from the beginning of string S such that it has at
  most K distinct characters.
2 Delete the selected substring in step-1 from string S

  You are allowed to change at most 1 character.

Task

Determine the maximum count of substrings that can be formed until the string S becomes
empty.

Notes
  • O-based indexing is used.
  • A substring is defined as a continuous sequence of characters of a string.

Example

Assumptions
  • N = 5
  • K = 2
  • S = aaaaa

Approach
Without changing any character in string S

• For string S = aaaaa
  The seelcted substring is "aaaaa" on deleting the substring, the string S= "" (empty string).

• Thus the answer is 1

On changing S[2]with character b'.
• For string S = aabaa
  * The selected substring is "aabaa" on deleting the substring, the string S= "" (empty string).
• Thus the answer is 1

Therefore the answer is 1

Function description
Complete the solve function provided in the editor. This function takes the following 3
parameters and returns the required answer:

• N: Represents the length of the array S
• K: Represents the given integer K
• S: Represents the string S

Input Format
Note: This is the input format you must use to provide custom input (available above the Compile and Test button).
• The first line contains T denoting the number of test cases. T also specifies the number 
  of times you have to run the solve function on a different set of inputs.
• For each test case:
  * The first line contains an integer N
  * The second line contains an integer K
  * The last line contains the string S

Output Format
For each test case in a new line, print an intege denoting the macximum count of substings that can be formed.

Constraints
1 <= T <= 10
1 <= N <= 10e4
1 <= K <= 26

S consists of lowercase English alphabets

Simple Input
2
5
2
aaaaa
7
3
abcccba

Sample Output
1
3


Explanation

The first line denotes the number of the test cases, T = -2.

The first test case
Explained in the problem statement example.

The second test cese

Given
  • N = 7
  * K = 3
  • S = abcccbd

Approach

Without changing any character in string S

• For string S = abcccba
    The selected substring is "abcccba: On deleting the substring the string S = "" (an empty string).
• Thus the answer is 1

On changing S[3] with character 'Z'.
  • For string S = abczcba.
    * The selected substring is "abc"_ On deleting the substring the string S = zcba.
  • For string S = zcba.
    * The selected substring is "zcb". On deleting the substring the string S = a.
  • For string S = a.
    * The selected substring is On deleting the substring the string S ="" (an empty string).
  • Thus the answer is 3

Therefore the answer is 3

Sample Input:
4
14
2
bacaaacccbaccd
5
2
zcfgu
7
2
eckpbhy
1
3
b

Sample Output
6
3
4
1

*/

use std::collections::HashSet;
use std::io::{self, prelude::*, BufRead, BufWriter};
use std::str::*;

struct Scanner<R> {
    reader: R,
    buf_str: Vec<u8>,
    buf_iter: SplitWhitespace<'static>,
}

impl<R: BufRead> Scanner<R> {
    fn new(reader: R) -> Self {
        Self {
            reader,
            buf_str: vec![],
            buf_iter: "".split_whitespace(),
        }
    }
    fn cin<T: FromStr>(&mut self) -> T {
        loop {
            if let Some(t) = self.buf_iter.next() {
                return t.parse().ok().expect("Failed to Parse!");
            }
            self.buf_str.clear();
            self.reader
                .read_until(b'\n', &mut self.buf_str)
                .ok()
                .expect("Failed to Read!");
            self.buf_iter = unsafe {
                let slice = from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_whitespace())
            }
        }
    }
}

fn generate_substrs(s: &Vec<char>, k: usize) -> usize {
    let mut index: usize = 0;
    let mut count: usize = 0;

    // let mut substrs: Vec<String> = Vec::new();

    while index < s.len() {
        let mut set: HashSet<char> = HashSet::new();
        // let mut temp_str: String = String::new();

        for i in index..s.len() {
            set.insert(s[i]);
            if set.len() <= k {
                // temp_str.push(s[i]);
                index = i + 1;
            } else {
                break;
            }
        }
        count += 1;
        // substrs.push(temp_str);
    }

    // return substrs;
    return count;
}

fn find_replace_index(s: &Vec<char>, start: usize) -> Option<usize> {
    let mut set: HashSet<char> = HashSet::new();

    for i in start..s.len() {
        if !set.insert(s[i]) {
            return Some(i);
        }
    }
    return None;
}

fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();

    let mut s = Scanner::new(stdin.lock());
    let mut out = BufWriter::new(stdout.lock());

    let mut t: usize = s.cin();

    while t > 0 {
        let (_n, k) = (s.cin::<usize>(), s.cin::<usize>());

        let string: String = s.cin();

        let mut string: Vec<char> = string.chars().collect();

        // let mut subs: Vec<String> = generate_substrs(&s, k);
        let mut count: usize = generate_substrs(&string, k);
        let mut start: Option<usize> = Some(0);

        let mut rep_place: Vec<usize> = Vec::new();

        // let mut count: usize = subs.len();

        while !start.is_none() {
            start = find_replace_index(&string, start.unwrap());
            if start.is_some() {
                rep_place.push(start.unwrap());
            }
        }

        // println!("{:?}", rep_place);
        // println!("{:?}", &subs);

        while !rep_place.is_empty() {
            let pos = rep_place.pop().unwrap();
            let ch: char = string[pos];
            string[pos] = '1';
            // subs = generate_substrs(&s, k);
            let cnt = generate_substrs(&string, k);
            // println!("{:?}", &subs);
            string[pos] = ch;
            count = std::cmp::max(count, cnt);
        }

        writeln!(out, "{}", count)?;

        t -= 1;
    }
    Ok(())
}
