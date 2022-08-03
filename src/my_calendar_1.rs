struct MyCalendar {
    records: Vec<(i32,i32)>
}

// 0729
// Brute Force
impl MyCalendar {

    fn new() -> Self {
        Self {
            records: vec![]
        }
    }
    
    fn book(&mut self, start: i32, end: i32) -> bool {
        if self.records.iter().find(|(s,e)| *s < end && *e > start).is_some() {
            false
        } else {
            self.records.push((start, end));
            true
        }
    }
}

