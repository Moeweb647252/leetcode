pub fn number_of_points(mut nums: Vec<Vec<i32>>) -> i32 {
    nums.sort_by(|v1, v2| v1[0].cmp(&v2[0]));
    let mut len = nums[0][1] - nums[0][0] + 1;
    let mut max = nums[0][1];
    for i in 1..nums.len() {
        let tmp = &nums[i];
        if tmp[1] > max {
            if tmp[0] <= max {
                len += tmp[1] - max;
                max = tmp[1]
            } else {
                len += tmp[1] - tmp[0] + 1;
                max = tmp[1]
            }
        }
    }
    len
}
