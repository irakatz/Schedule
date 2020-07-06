//同余方程
//https://www.luogu.com.cn/problem/solution/P1082
//练习运算
static mut X:i128=0;
static mut Y:i128=0;
fn main(){
unsafe {
    let mut allnum=String::new();
    std::io::stdin().read_line(&mut allnum).unwrap();
    let mut s=allnum.trim().split(' ');
    let mut bucket:[i128;2]=[0;2];
    let mut cnt:usize=0;
    for ix in s{
        bucket[cnt]=ix.trim().parse::<i128>().unwrap();
        cnt+=1;
    }
    extendgcd(bucket[0],bucket[1]);
    X=(X+bucket[1])%bucket[1];
    println!("{}",X);
}


}
unsafe fn extendgcd(a:i128, b:i128){
    if(b==0){X=1;Y=0;}
    else{
        extendgcd(b,a%b);
        let mut prevX=X;
        X=Y;
        Y=prevX-a/b*Y;


    }

}
