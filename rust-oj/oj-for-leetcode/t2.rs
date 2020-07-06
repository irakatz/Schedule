//4的幂
/*
给定一个整数 (32 位有符号整数)，请编写一个函数来判断它是否是 4 的幂次方。
*/
//https://leetcode-cn.com/problems/power-of-four/
//rust实现
impl Solution {
    pub fn is_power_of_four(num: i32) -> bool {
let b=0x55555555;
        if(num<0||(num&(num-1))!=0){return false;}
if((num&b)==0){return false;}else{return true;}
    }
}
//非rust实现
// class Solution {
// public boolean isPowerOfFour(int num) {
// int b=0x55555555;
// if(num<0||(num&(num-1))!=0)return false;
// if((num&b)==0)return false;else return true;
// }
//
// }