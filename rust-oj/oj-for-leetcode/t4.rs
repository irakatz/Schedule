// 回文质数
//因为 151 既是一个质数又是一个回文数（从左到右和从右到左是看一样的），所以 151 是回文质数。
//写一个程序来找出范围 [a,b] (5≤a<b≤100,000,000)( 一亿)间的所有回文质数。
// https://www.luogu.com.cn/problem/P1217
//rust实现
fn main(){
    const MAX:usize=10000001;
    let mut allnum:[u32;MAX]=[1;MAX];
    allnum[0]=0;
    allnum[1]=0;
    for i in 2..MAX{
        if allnum[i]==0{continue}
        else{
            let mut j:i64=i as i64*2;
            while j<MAX as i64 {
                allnum[j as usize]=0;
                j+=i as i64;
            }
        }
    }
    let mut input=String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut s=input.trim().split(' ');

    let mut a:usize=s.next().unwrap()
        .parse().unwrap();
    let mut b:usize=s.next().unwrap()
        .parse().unwrap();
    if b>MAX{b=MAX-5;}
    for i in a..(b+1+{
        if allnum[i]==0{continue}
        let strv=i.to_string();
        let mut lt=0;let mut rt=strv.len()-1;
        let charsstrv: Vec<char> = strv.chars().collect();
        let mut tf=true;
        while lt<rt {
            if charsstrv[lt]!=charsstrv[rt]{tf=false}
            lt+=1;
            rt-=1;
        }
        if tf{println!("{}",i)}
    }
}
//非rust实现

//#include<stdio.h>
//#include <iostream>
//#include <string>
//using namespace std;
//int main(){
//    int MAX=10000001;
//    int a;
//    int b;
//    scanf("%d %d",&a,&b);
//    if (a>MAX){a=MAX;}
//    if (b>MAX){b=MAX-5;}
//    int allnum[MAX];
//    for(int i=0;i<MAX;i++){allnum[i]=1;}
//    allnum[0]=0;
//    allnum[1]=0;
//    for(int i=2;i<MAX;i++){
//        if(allnum[i]!=0){
//            int j=2*i;
//            while(j<MAX){
//                allnum[j]=0;
//                j+=i;
//            }
//        }
//    }
//    for(int i=a;i<=b;i++){
//        if(allnum[i]!=0){
//            string s=to_string(allnum[i]);
//            int lt=0;
//            int rt=s.length()-1;
//            bool tf=true;
//            while(lt<rt){
//                if(s[lt]!=s[rt]){tf=false;}
//                lt+=1;
//                rt-=1;
//            }
//            if(tf){printf("%d\n",i);}
//        }
//        }
//
//}