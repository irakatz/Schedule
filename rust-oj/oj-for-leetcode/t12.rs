//删列造序
//https://leetcode-cn.com/problems/delete-columns-to-make-sorted/
/*
给定由 N 个小写字母字符串组成的数组 A，其中每个字符串长度相等。
你需要选出一组要删掉的列 D，对 A 执行删除操作，使 A 中剩余的每一列都是 非降序 排列的，然后请你
返回 D.length 的最小可能值。
删除 操作的定义是：选出一组要删掉的列，删去 A 中对应列中的所有字符，形式上，第 n 列为 [A[0][n],
 A[1][n], ..., A[A.length-1][n]]）。（可以参见 删除操作范例）
 */

//rust
impl Solution {
    pub fn min_deletion_size(a: Vec<String>) -> i32 {
        if a.is_empty()||a.len()==1{return 0;}
        let mut aa=0;
        for i in 0..a[0].len(){
            for j in 0..a.len()-1{
                if(a[j].chars().nth(i)>a[j+1].chars().nth(i)){aa+=1;break;}
            }
        }
        return aa;
    }
}
//非rust
// class Solution {
// public int minDeletionSize(String[] A) {
// if(A==null)return 0; int a=0;
// if(A.length<=1)return 0;
// for(int i=0;i<A[0].length();i++){
// for(int j=0;j<A.length-1;j++){
// if(A[j].charAt(i)>A[j+1].charAt(i))
// {a++;
// break;}
// }
//
// }
// return a;
//
// }
// }