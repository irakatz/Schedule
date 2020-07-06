//4. 寻找两个正序数组的中位数
//https://leetcode-cn.com/problems/median-of-two-sorted-arrays/
//给定两个大小为 m 和 n 的正序（从小到大）数组 nums1 和 nums2。
// 请你找出这两个正序数组的中位数，并且要求算法的时间复杂度为 O(log(m + n))。
//rust
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut m=0;let mut n=0;let mut jo=0;
        let mut newnums1=nums1.clone();;let mut newnums2=nums2.clone();
        if(nums1.len()>nums2.len()){ newnums1=nums2.clone();newnums2=nums1.clone(); }
        m=newnums1.len();n=newnums2.len();

        if((m+n)%2==1){jo=1;}else {jo=0;}
        if(m==0){if(jo==0){
            let mut rt=n/2;let mut lt=rt-1;
            return (newnums2[rt] + newnums2[lt]) as f64/2.0 ;
        }
        else{
            return newnums2[n / 2] as f64 ;
        }

        }
        if(n==0){if(jo==0){
            let mut rt=m/2;let mut lt=rt-1;
            return (newnums1[rt]+newnums1[lt]) as f64/2.0 ;
        }
        else{
            return newnums1[m / 2] as f64 ;
        }

        }
        let mut il=0;let mut ir=m;
        while(il<=ir){
            let mut i=(il+ir)/2;
            let mut j=(m+n+1)/2-i;
            //考虑i,j是0的事
            if(i<m&&j>0&&newnums2[j-1]>
                newnums1[i]){
                il=i+1;
            }
            else if(j<n&&i>0&&newnums1[i-1]>newnums2[j]){ir=i-1;}
            else {
                let mut ltmid=0;let mut rtmid=0;
                if(j==0){ltmid=newnums1[i-1];}
                else if(i==0){ltmid=newnums2[j-1];}
                else{ltmid=std::cmp::max(newnums1[i-1],newnums2[j-1]);}

                if(i==m){rtmid=newnums2[j];}
                else if(j==n){rtmid=newnums1[i];}
                else{rtmid=std::cmp::min(newnums1[i],newnums2[j]);}
                if(jo==1){return ltmid as f64 ; }else {return(ltmid+rtmid) as f64/2.0 ;}

            }

        }
        return 0 as f64;
    }
}
//非rust
// class Solution {
// public double findMedianSortedArrays(int[] nums1, int[] nums2) {
// int m;int n;int jo;
// if(nums1.length<nums2.length||nums1.length==nums2.length){m=nums1.length;n=nums2.length;}
// else{int []tmp=nums1;nums1=nums2;nums2=tmp;m=nums1.length;n=nums2.length;}
//
// if((m+n)%2==1){jo=1;}else jo=0;
// if(m==0){if(jo==0){
// int rt=n/2;int lt=rt-1;return (nums2[rt]+nums2[lt])/2.0;
// }
// else{
// return nums2[n/2];
// }
//
// }
// if(n==0){if(jo==0){
// int rt=m/2;int lt=rt-1;return (nums1[rt]+nums1[lt])/2.0;
// }
// else{
// return nums1[m/2];
// }
//
// }
// int il=0;int ir=m;
// while(il<=ir){
// int i=(il+ir)/2;int j=(m+n+1)/2-i;
// //考虑i,j是0的事
// if(i<m&&j>0&&nums2[j-1]>nums1[i]){
// il=i+1;
// }
// else if(j<n&&i>0&&nums1[i-1]>nums2[j]){ir=i-1;}
// else {
// int ltmid=0;int rtmid=0;
// if(j==0){ltmid=nums1[i-1];}
// else if(i==0){ltmid=nums2[j-1];}
// else{ltmid=Math.max(nums1[i-1],nums2[j-1]);}
//
// if(i==m){rtmid=nums2[j];}
// else if(j==n){rtmid=nums1[i];}
// else{rtmid=Math.min(nums1[i],nums2[j]);}
// if(jo==1)return ltmid;else return (ltmid+rtmid)/2.0;
//
// }
//
// }
// return 0;
// }
// }