//摆动序列
//https://leetcode-cn.com/problems/wiggle-subsequence/
/*
如果连续数字之间的差严格地在正数和负数之间交替，则数字序列称为摆动序列。第一个差（如果存在的话）
可能是正数或负数。少于两个元素的序列也是摆动序列。
例如， [1,7,4,9,2,5] 是一个摆动序列，因为差值 (6,-3,5,-7,3) 是正负交替出现的。相反, [1,4,7,2,5] 
和 [1,7,4,5,5] 不是摆动序列，第一个序列是因为它的前两个差值都是正数，第二个序列是因为它的最后一个差值为零。
给定一个整数序列，返回作为摆动序列的最长子序列的长度。 通过从原始序列中删除一些（也可以不删除）元素来获得
子序列，剩下的元素保持其原始顺序。
*/
//rust
impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        let mut cnt=1;let mut t=false;
        if(nums.len()<2){return nums.len() as i32;}
        let mut start=0;let mut beforetf=0;
        for i in 1..nums.len(){
            if(nums[i]!=nums[0]){
                beforetf=nums[i]-nums[0];
                if(nums[i]>nums[0]){start=i;t=true;cnt+=1;break;}
                else{start=i;cnt+=1;break;}
            }
        }
        for i in start+1..nums.len(){
            let mut nowtf=nums[i]-nums[i-1];
            if(nowtf==0){continue;}
            else if(nowtf>0){
                if(!t){t=true;cnt+=1;}
            }
            else{
                if(t){t=false;cnt+=1;}
            }
        }
return cnt;
    }
}
//非rust
// class Solution {
// public int wiggleMaxLength(int[] nums) {
// int cnt=1;boolean t=false;
// if(nums.length<2)return nums.length;
//
// int start=0;int beforetf=0;
// for(int i=1;i<nums.length;i++){
// if(nums[i]!=nums[0])
// {beforetf=nums[i]-nums[0];
// if(nums[i]>nums[0]){start=i;t=true;cnt++;break;}
// else{start=i;cnt++;break;}
// }
// }
//
// for(int i=start+1;i<nums.length;i++){
// int nowtf=nums[i]-nums[i-1];
// if(nowtf==0)continue;
// else if(nowtf>0){
// if(!t){t=true;
// cnt++;
// }
// }
// else{
// if(t){t=false;cnt++;}
//
// }
//
// }
// return cnt;
// }
// }