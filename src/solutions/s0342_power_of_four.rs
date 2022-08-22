// https://leetcode.com/problems/power-of-four/

use crate::solutions::Solution;

impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        let mut devide_result = n as f32;
        
        while devide_result > 1f32 {
            devide_result /= 4f32;
        }
        
        devide_result == 1f32
    }
}

#[cfg(test)]
mod tests{
    use test_case::test_case;

    #[test_case(16, true)]
    #[test_case(12, false)]
    #[test_case(5, false)]
    #[test_case(1, true)]
    #[test_case(0, false)]
    #[test_case(-1, false)]
    #[test_case(-4, false)]
    #[test_case(-8, false)]
    #[test_case(-16, false)]
    fn success_cases(n: i32, expected: bool){
        assert_eq!(super::Solution::is_power_of_four(n), expected);
    }
}
