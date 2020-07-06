//45.跳跃游戏2
//https://leetcode-cn.com/problems/jump-game-ii/submissions/
/*
给定一个非负整数数组，你最初位于数组的第一个位置。
数组中的每个元素代表你在该位置可以跳跃的最大长度。
你的目标是使用最少的跳跃次数到达数组的最后一个位置。
*/
//rust实现
use std::cmp::max;
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        if(nums.len()<2){return 0;}
        let mut step=0;
        let mut next=nums[0];
        let mut reach=0;
        let len=nums.len();
        for mut i in 0..len{
            next=std::cmp::max((i + nums[i] as usize) as i32, next);
            if(next>= (len - 1) as i32){return step+1;}
            if(i==reach){step+=1;reach= next as usize;}
        }
        return 0;
    }
}
//非rust实现
// class Solution {
// public int jump(int[] nums) {
// if(nums.length<2)return 0;
// int step=0;int next=nums[0];int reach=0;
// for(int i=0;i<nums.length;i++){
// next=Math.max(i+nums[i],next);
// if(next>=nums.length-1)return step+1;
// if(i==reach){
// step++;reach=next;
// }
// }
// return step;
// }
// }
