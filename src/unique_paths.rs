use crate::solution::Solution;

impl Solution {
  // paths(m,n)
  //   if m < 0 or n < 0 return 0
  //   if m == 1 and n == 1 return 1
  //   return paths(m-1, n) + paths(m, n+1)
  
  pub fn unique_paths(m: i32, n: i32) -> i32 {
    if m < 0 || n < 0 {
      return 0;
    }

    let mut map = vec![vec![0i32; (m+1) as usize]; (n+1) as usize];
    
    for y in 1..(n+1) as usize {
      for x in 1..(m+1) as usize {
        if x == 1 && y == 1 { 
          map[1][1] = 1;
        } else {
          map[y][x] = map[y-1][x] + map[y][x-1];
        }
      }
    }
    map[n as usize][m as usize]
  }
}


#[cfg(test)]
mod tests {
  use super::*;
  use test_case::test_case;

  #[test_case(2,2,2)]
  #[test_case(3,7,28)]
  #[test_case(3,2,3)]
  fn success_cases(m:i32, n:i32, result: i32) {
    assert_eq!(Solution::unique_paths(m,n), result);
  }
}