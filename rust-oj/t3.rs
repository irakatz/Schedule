// 明明的随机数
// https://www.luogu.com.cn/problem/P1059
// 桶排序
fn main(){
    let mut input=String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let num=input.trim().parse::<i32>().unwrap();
    let mut allnum=String::new();
    std::io::stdin().read_line(&mut allnum).unwrap();
    let mut s=allnum.trim().split(' ');
//    let mut vecs:Vec<i32> = Vec::new();
    let mut bucket:[i32;1001]=[0;1001];

    for ix in s{
        let mut j=ix.trim().parse::<i32>().unwrap();
        // vecs.push(j);
        bucket[j as usize]=1;
    }
    let mut cnt=0;
    for i in 0..1001{
        if bucket[i as usize]==1{cnt+=1;}
    }
    println!("{}",cnt);
    for i in 0..1001{
        if bucket[i as usize]==1{print!("{} ",i);}
    }
}
