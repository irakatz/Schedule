//删除排序数组中的重复项
//https://leetcode-cn.com/problems/remove-duplicates-from-sorted-array/
/*
给定一个排序数组，你需要在 原地 删除重复出现的元素，使得每个元素只出现一次，返回移除后数组的新长度。

不要使用额外的数组空间，你必须在 原地 修改输入数组 并在使用 O(1) 额外空间的条件下完成。
*/
//rust
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if(nums.len()==0){0}
        else if (nums.len()==1){1}
        let mut top=0;
        let mut i=0;
        let mut j=0;
        let mut count=1;
        while(i<nums.len()){
            if(nums[i]!=nums[j]){count+=1;top+=1;nums[top]=nums[i];}
            j=i;i+=1;
        }
        count;
    }
}
//非rust
// class Solution {
// public int removeDuplicates(int[] nums) {
// if(nums.length==0)return 0;
// else if(nums.length==1)return 1;
// int top=0;
// int i=1;int j=0;int count=1;
// while(i<nums.length){
// if(nums[i]!=nums[j]){count++;top++;nums[top]=nums[i];}
// j=i;i++;
// }
// return count;
// }
// }