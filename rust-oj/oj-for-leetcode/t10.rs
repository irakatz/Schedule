//122. 买卖股票的最佳时机 II
//给定一个数组，它的第 i 个元素是一支给定股票第 i 天的价格。
// 设计一个算法来计算你所能获取的最大利润。你可以尽可能地完成更多的交易（多次买卖一支股票）。
//https://leetcode-cn.com/problems/best-time-to-buy-and-sell-stock-ii/
//rust
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max=Vec::new();
        let mut min =Vec::new();

        if(prices.len()==0){return 0;}
        if(prices.len()==1){return 0;}
        else if(prices.len()==2){
            if(prices[1]>prices[0]){return prices[1]-prices[0];}else {return 0;}
        }
        if(prices[0]<prices[1]){min.push(prices[0]);}
        for i in 1..prices.len()-1{
            if((prices[i]<prices[i-1]||prices[i]==prices[i-1])&&prices[i]<prices[i+1]){min.push(prices[i]);}
            if(prices[i]>prices[i-1]&&(prices[i]>prices[i+1]||prices[i]==prices[i+1])){max.push(prices[i]);}
        }
        if(prices[prices.len()-1]>prices[prices.len()-2]){max.push(prices[prices.len()-1]);}
        let mut cnt=0;

        for i in 0..min.len(){
            cnt=cnt+(max[i]-min[i]);
        }
        return cnt;
    }
}
//非rust
// class Solution {
// public int maxProfit(int[] prices) {
// ArrayList<Integer>max=new ArrayList<>();
// ArrayList<Integer>min=new ArrayList<>();
// if(prices.length==0)return 0;
// if(prices.length==1)return 0;
// else if(prices.length==2){
// if(prices[1]>prices[0])return prices[1]-prices[0];else return 0;
// }
// if(prices[0]<prices[1]){min.add(prices[0]);}
// for(int i=1;i<prices.length-1;i++){
// if((prices[i]<prices[i-1]||prices[i]==prices[i-1])&&prices[i]<prices[i+1])min.add(prices[i]);
// if(prices[i]>prices[i-1]&&(prices[i]>prices[i+1]||prices[i]==prices[i+1]))max.add(prices[i]);
// }
// if(prices[prices.length-1]>prices[prices.length-2])max.add(prices[prices.length-1]);
// int cnt=0;
//
// for(int i=0;i<min.size();i++){
// cnt=cnt+(max.get(i)-min.get(i));
// }
// return cnt;
// }
// }