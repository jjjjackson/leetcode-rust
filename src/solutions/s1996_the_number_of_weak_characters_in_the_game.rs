use crate::solutions::Solution;

impl Solution {
    pub fn number_of_weak_characters(properties: Vec<Vec<i32>>) -> i32 {
        use std::cmp::Ordering;

        
        let mut p = properties;
        p.sort_by(|a,b| {
            if ( a[0] == b[0] && a[1] > b[1] ) || a[0] < b[0] {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        });
        
        let mut max_till_now = i32::MIN;
        let mut result = 0;
        
        for i in p {
            if i[1] < max_till_now {
                result += 1 ;
            }
            max_till_now = max_till_now.max(i[1]);
        }
        
        result
    }
}
