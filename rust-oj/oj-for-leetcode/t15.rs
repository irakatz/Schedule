//921. 使括号有效的最少添加
//https://leetcode-cn.com/problems/minimum-add-to-make-parentheses-valid/
/*
给定一个由 '(' 和 ')' 括号组成的字符串 S，我们需要添加最少的括号（ '(' 或是 ')'，可以在任何位置）
，以使得到的括号字符串有效。
*/
//rust
impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut ss=0;
        let mut cnt=0;
        for i in 0..s.len(){
            let mut t=s.chars().nth(i).unwrap();
            if ss==0{
                if t==')'{cnt+=1;}
                else{ss+=1;}
            }
            else{
                if t=='('{ss+=1;}
                else{ss-=1;}
            }
        }
        return cnt+ss;
    }
}
//非Rust
// class Solution {
// public int minAddToMakeValid(String S) {
// int ss=0;
// int cnt=0;
// for(int i=0;i<S.length();i++){
// char t=S.charAt(i);
// if(ss==0){
// if(t==')')cnt++;
// else ss++;
// }
// else{
// if(t=='(')ss++;
// else ss--;
// }
// }
// return cnt+ss;
// }
// }