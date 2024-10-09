use super::Solution;

/**

有 n 个人前来排队买票，其中第 0 人站在队伍 最前方 ，第 (n - 1) 人站在队伍 最后方 。

给你一个下标从 0 开始的整数数组 tickets ，数组长度为 n ，其中第 i 人想要购买的票数为 tickets[i] 。

每个人买票都需要用掉 恰好 1 秒 。一个人 一次只能买一张票 ，如果需要购买更多票，他必须走到  队尾 重新排队（瞬间 发生，不计时间）。如果一个人没有剩下需要买的票，那他将会 离开 队伍。

返回位于位置 k（下标从 0 开始）的人完成买票需要的时间（以秒为单位）。

https://leetcode.cn/problems/time-needed-to-buy-tickets/description/?envType=daily-question&envId=2024-09-29

 */

impl Solution {
    #[allow(dead_code)]
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let tk = tickets[k];
        tickets
            .iter()
            .enumerate()
            .map(|(i, &t)| t.min(if i <= k { tk } else { tk - 1 }))
            .sum()
    }
}