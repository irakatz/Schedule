//分发饼干
//https://leetcode-cn.com/problems/assign-cookies/
/*
假设你是一位很棒的家长，想要给你的孩子们一些小饼干。但是，每个孩子最多只能给一块饼干。对每个孩子 i ，都有
一个胃口值 gi ，这是能让孩子们满足胃口的饼干的最小尺寸；并且每块饼干 j ，都有一个尺寸 sj 。如果 sj >= gi 
，我们可以将这个饼干 j 分配给孩子 i ，这个孩子会得到满足。你的目标是尽可能满足越多数量的孩子，
并输出这个最大数值。
*/
impl Solution {
    pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
        let mut gx=g.clone();
        let mut sx=s.clone();
        gx.sort();
        sx.sort();
        let mut gi=0;
        let mut si=0;
        let mut cnt=0;
        while(true){
            if si==sx.len(){break;}
            if gi==gx.len(){break;}
            println!("{},{}",gx[gi],sx[si]);
            if(gx[gi]<=sx[si]){
                cnt+=1;
                si+=1;
                gi+=1;
            }
            else{
                si+=1;
            }
        }
        return cnt;
    }
}
//rust
//非Rust
// class Solution {
// public int findContentChildren(int[] g, int[] s) {
// Arrays.sort(g);Arrays.sort(s);
// int gi=0;int si=0;int cnt=0;
// while(true){
// if(si==s.length)break;
// if(gi==g.length)break;
// if(!(s[si]<g[gi])){cnt++;gi++;si++;}else{si++;}
// }
// return cnt;
// }
// }