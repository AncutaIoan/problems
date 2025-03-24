impl Solution {
    pub fn count_days(days: i32, meetings: Vec<Vec<i32>>) -> i32 {
        let mut meetings = meetings;
        meetings.sort_by_key(|v| v[0]);

        let mut merged_meetings: Vec<Vec<i32>> = Vec::new();

        for meet in meetings {
            if merged_meetings.is_empty() {
                merged_meetings.push(meet);
            } else {
                let last = merged_meetings.last_mut().unwrap();
                if last[1] >= meet[0] {
                    last[1] = last[1].max(meet[1]);
                } else {
                    merged_meetings.push(meet);
                }
            }
        }

        let mut free_days = 0;

        if merged_meetings[0][0] > 1 {
            free_days += merged_meetings[0][0] - 1;
        }

        for i in 1..merged_meetings.len() {
            let gap = merged_meetings[i][0] - merged_meetings[i - 1][1] - 1;
            if gap > 0 {
                free_days += gap;
            }
        }

        if merged_meetings.last().unwrap()[1] < days {
            free_days += days - merged_meetings.last().unwrap()[1];
        }

        return free_days;
    }
}
