/*
2528. Maximize the Minimum Powered City

You are given a 0-indexed integer array stations of length n, where stations[i] represents the number of power stations in the ith city.

Each power station can provide power to every city in a fixed range. In other words, if the range is denoted by r, then a power station at city i can provide power to all cities j such that |i - j| <= r and 0 <= i, j <= n - 1.

Note that |x| denotes absolute value. For example, |7 - 5| = 2 and |3 - 10| = 7.
The power of a city is the total number of power stations it is being provided power from.

The government has sanctioned building k more power stations, each of which can be built in any city, and have the same range as the pre-existing ones.

Given the two integers r and k, return the maximum possible minimum power of a city, if the additional power stations are built optimally.

Note that you can build the k power stations in multiple cities.
*/
impl Solution {
    pub fn max_power(stations: Vec<i32>, r: i32, k: i32) -> i64 {
        let n = stations.len();
        let mut left: i64 = *stations.iter().min().unwrap() as i64;
        let mut right: i64 = stations.iter().map(|&x| x as i64).sum::<i64>() + k as i64 + 1;

        let mut stations_ref = stations.clone();
        while left < right {
            let mid = (left + right) / 2;
            if Self::check(&stations_ref, r as usize, k as i64, mid) {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left - 1
    }

    fn check(stations: &Vec<i32>, r: usize, mut additional: i64, min_power: i64) -> bool {
        let n = stations.len();
        let mut stations = stations.iter().map(|&x| x as i64).collect::<Vec<_>>();
        let mut power: i64 = 0;

        // Initialize power with first r cities (like accumulate up to r-1)
        for i in 0..r.min(n) {
            power += stations[i];
        }

        for i in 0..n {
            if i + r < n {
                power += stations[i + r];
            }

            if power < min_power {
                let need = min_power - power;
                if need > additional {
                    return false;
                }
                additional -= need;

                // Plant new stations at the farthest possible city (i + r)
                let pos = std::cmp::min(n - 1, i + r);
                stations[pos] += need;
                power += need;
            }

            if i >= r {
                power -= stations[i - r];
            }
        }

        true
    }
}
