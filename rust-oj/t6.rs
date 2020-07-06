//肥胖问题
//https://www.luogu.com.cn/problem/P5714
//对浮点数的应用
//Rust暂时没有对浮点数规定输出的数字格式。同时，我也没有找到四舍五入的函数
fn main(){
    let mut allnum=String::new();
    std::io::stdin().read_line(&mut allnum).unwrap();
    let mut s=allnum.trim().split(' ');
    let mut bucket:[f32;2]=[0.0;2];
    let mut j:usize=0;
    for ix in s{
        bucket[j]=ix.trim().parse::<f32>().unwrap();
        j+=1;
    }
    let bmi=bucket[0]/(bucket[1]*bucket[1]);
    if bmi<18.5{println!("Underweight");}
    else if bmi>=18.5&&bmi<24.0{println!("Normal");}
    else{
        
        println!("{:.4}",bmi);
        println!("Overweight");
    }
}

