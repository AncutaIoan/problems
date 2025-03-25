impl Solution {
    pub fn check_valid_cuts(n: i32, rectangles: Vec<Vec<i32>>) -> bool {
        fn merge_ranges(ranges: &mut Vec<Vec<i32>>) -> usize {
            ranges.sort_by_key(|v| v[0]);
            let mut merged = vec![ranges[0].clone()];
            for r in &ranges[1..] {
                let last = merged.last_mut().unwrap();
                if last[1] > r[0] {
                    last[1] = last[1].max(r[1]);
                } else {
                    merged.push(r.clone());
                }
            }
            merged.len()
        }

        let mut x_ranges = Vec::new();
        let mut y_ranges = Vec::new();

        for rect in rectangles {
            x_ranges.push(vec![rect[0], rect[2]]);
            y_ranges.push(vec![rect[1], rect[3]]);
        }

        merge_ranges(&mut x_ranges) >= 3 || merge_ranges(&mut y_ranges) >= 3
    }
}

/* Optim solution */ 
/*
impl Solution {
    pub fn check_valid_cuts(n: i32, mut rectangles: Vec<Vec<i32>>) -> bool {
        Solution::merge(&mut rectangles,0) >= 3 || Solution::merge(&mut rectangles,1) >= 3
    }

    fn merge(rectangles:&mut Vec<Vec<i32>>,start: usize) -> i32 {
        rectangles.sort_by_key(|item| item[start]);        
        rectangles.iter().enumerate().skip(1).fold((1,rectangles[0][start],rectangles[0][start+2]),|(result,first,second),(idx,item)| {
            if item[start] >= first && item[start] < second {
                (result,first,second.max(item[start+2]))                
            } else {
                (result+1,item[start],item[start+2])
            }
        }).0        
    }
}
*/
