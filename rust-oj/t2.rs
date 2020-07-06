// 二叉树深度
// https://www.luogu.com.cn/problem/P4913
// 实际上这个题只能通过一些基本的测试，对于大量数据会有MLE
static  mut MAXIMUM:i32=-1;

unsafe fn dfs(node:i32, length:i32, tree:[[i32;2];10000]){
    if node==0{return}
    MAXIMUM=std::cmp::max(MAXIMUM,length);
    dfs(tree[node as usize][0],length+1,tree);
    dfs(tree[node as usize][1],length+1,tree);
}
fn main(){
    unsafe {
        let mut input=String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let num=input.trim().parse::<i32>().unwrap();
        let mut tree:[[i32;2];10000] = [[0;2];10000];
        for i  in 1..num+1{
            let mut input2=String::new();
            std::io::stdin().read_line(&mut input2).unwrap();
            let mut s=input2.trim().split(' ');
            // for ix in s{
            //     tree[i]=ix.trim().parse::<i32>().unwrap();
            // }
            let mut num1=s.next().unwrap().parse::<i32>().unwrap();
            let mut num2=s.next().unwrap().parse::<i32>().unwrap();
            tree[i as usize][0]=num1;
            tree[i as usize][1]=num2;
        }
        // for i in 1..num+1{
        //     println!("{} {}",tree[i as usize][0],tree[i as usize][1]);
        // }
        dfs(1,1,tree);
        println!("{}",MAXIMUM)
    }

}
