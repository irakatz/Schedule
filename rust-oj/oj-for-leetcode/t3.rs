use std::intrinsics::atomic_singlethreadfence;

//50. Pow(x, n)
//leetcode-cn.com/problems/powx-n/
//实现 pow(x, n) ，即计算 x 的 n 次幂函数。
//rust实现
impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let mut ans:f64=1.0;
        let mut i=n;
        let mut xx=x;
        while i!=0{
            if(i&1!=0){ans*=xx;}
            xx*=xx;
            i/=2;
        }
            if n<0{
                return 1.0/ans;
            }
            else{
                return ans;
            }
    }
}
//非rust实现
// class Solution {
// public:
// double myPow(double x, int n) {
// double ans=1;
// for(int i=n;i!=0;i/=2){
// if(i&1!=0)ans*=x;
// x*=x; }
// return n<0?1/ans:ans;
// }
// };