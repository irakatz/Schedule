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
    if a>MAX{a=MAX;}
    if b>MAX{b=MAX;}
    for i in a..b{
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

