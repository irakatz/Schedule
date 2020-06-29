// //T0.转大写
// fn main(){
//     let mut input=String::new();
//     std::io::stdin().read_line(&mut input).unwrap();
//     let scalars: Vec<char> = input.chars().collect();
//     for i in 0..scalars.len(){
//         print!("{}",&scalars[i].to_uppercase())
//     }
//     println!()
// }
//     //T1.回文质数
// fn main(){
//     const MAX:usize=10000001;
//     let mut allnum:[u32;MAX]=[1;MAX];
//     allnum[0]=0;
//     allnum[1]=0;
//     for i in 2..MAX{
//         if allnum[i]==0{continue}
//         else{
//             let mut j:i64=i as i64*2;
//             while j<MAX as i64 {
//                 allnum[j as usize]=0;
//                 j+=i as i64;
//             }
//         }
//     }
//     let mut input=String::new();
//     std::io::stdin().read_line(&mut input).unwrap();
//     let mut s=input.trim().split(' ');
//
//     let mut a:usize=s.next().unwrap()
//         .parse().unwrap();
//     let mut b:usize=s.next().unwrap()
//         .parse().unwrap();
//     if a>MAX{a=MAX;}
//     if b>MAX{b=MAX;}
//     for i in a..b{
//         if allnum[i]==0{continue}
//         let strv=i.to_string();
//         let mut lt=0;let mut rt=strv.len()-1;
//         let charsstrv: Vec<char> = strv.chars().collect();
//         let mut tf=true;
//         while lt<rt {
//             if charsstrv[lt]!=charsstrv[rt]{tf=false}
//             lt+=1;
//             rt-=1;
//         }
//         if tf{println!("{}",i)}
//     }
// }
