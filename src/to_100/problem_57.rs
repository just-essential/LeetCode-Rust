/*
Given a set of non-overlapping intervals, insert a new interval into the intervals (merge if necessary).

You may assume that the intervals were initially sorted according to their start times.

Example 1:

Input: intervals = [[1,3],[6,9]], newInterval = [2,5]
Output: [[1,5],[6,9]]
Example 2:

Input: intervals = [[1,2],[3,5],[6,7],[8,10],[12,16]], newInterval = [4,8]
Output: [[1,2],[3,10],[12,16]]
Explanation: Because the new interval [4,8] overlaps with [3,5],[6,7],[8,10].
NOTE: input types have been changed on April 15, 2019. Please reset to default code definition to get new method signature.
*/
struct Solution;
impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut new_interval = new_interval;
        let mut result = vec![];
        for interval in intervals {
            if new_interval[1] < interval[0] {
                result.push(new_interval);
                new_interval = interval.clone();
            }
            if interval[1] < new_interval[0] {
                result.push(interval);
            } else {
                new_interval[0] = new_interval[0].min(interval[0]);
                new_interval[1] = new_interval[1].max(interval[1]);
            }
        }
        result.push(new_interval);
        result
    }
//    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
//        if intervals.is_empty() {
//            return vec![new_interval];
//        }
//        let mut result = vec![];
//        for interval in &intervals {
//            if interval[0] > new_interval[0] {
//                break;
//            }
//            result.push(interval.clone());
//        }
//        let mut flag = result.len();
//        let len = intervals.len();
//        if flag == 0 || new_interval[0] > result[flag - 1][1] {
//            result.push(new_interval.clone());
//        } else if result[flag - 1][1] < new_interval[1] {
//            result[flag - 1][1] = new_interval[1];
//        }
//        while flag < len {
//            if intervals[flag][0] <= new_interval[1] {
//                if intervals[flag][1] > new_interval[1] {
//                    result.last_mut().unwrap()[1] = intervals[flag][1];
//                }
//            } else {
//                result.push(intervals[flag].clone());
//            }
//            flag += 1;
//        }
//        result
//    }
}
#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn example_1() {
        assert_eq!(
            Solution::insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5]),
            vec![vec![1, 5], vec![6, 9]]
        );
    }
    #[test]
    fn example_2() {
        assert_eq!(
            Solution::insert(
                vec![
                    vec![1, 2],
                    vec![3, 5],
                    vec![6, 7],
                    vec![8, 10],
                    vec![12, 16]
                ],
                vec![4, 8]
            ),
            vec![vec![1, 2], vec![3, 10], vec![12, 16]]
        );
    }
}
