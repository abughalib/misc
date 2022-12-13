struct Solution;

impl Solution {

  fn are_same(s1: &[char], uset: &Vec<usize>) -> bool {

    let mut v1: Vec<usize> = vec![0; 26];

    for ch in s1.iter() {
      v1[*ch as usize - 'a' as usize]+= 1;
    }

    println!("{:?}\n{:?}", s1, uset);

    for i in 0..s1.len() {
      if v1[i] != uset[i] {
        return false;
      }
    }

    return true;
  }

  pub fn check_inclusion(s1: String, s2: String) -> bool {
    let s1: Vec<char> = s1.trim().chars().collect();
    let s2: Vec<char> = s2.trim().chars().collect();

    let mut vec: Vec<usize> = vec![0; 26];

    for ch in s1.iter() {
      vec[*ch as usize - 'a' as usize]+=1;
    }

    if s1.len() > s2.len() {
      return false;
    }

    if s1.len() == s2.len() {
      return Solution::are_same(&s1, &vec);
    }

    let mut left: usize = 0;
    let mut right: usize = s1.len();

    while right <= s2.len() {
      if Solution::are_same(&s2[left..right], &vec) {
        return true;
      }
      left += 1;
      right += 1;
    }

    return false;
  }
}

fn main() {
  
  let str1 = String::from("hello");
  let str2 = String::from("ooolleoooleh");

  println!("{}", Solution::check_inclusion(str1, str2));

}
