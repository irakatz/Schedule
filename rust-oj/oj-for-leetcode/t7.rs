// 二叉树深度
// https://www.luogu.com.cn/problem/P4913
//给出每个节点的两个儿子节点，建立一棵二叉树（根节点为 11），如果是叶子节点，
//则输入0 0。建好树后希望知道这棵二叉树
//的深度。二叉树的深度是指从根节点到叶子结点时，最多经过了几层。
// 实际上这个题只能通过一些基本的测试，对于大量数据会有MLE
//rust
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
//非rust
#include<stdio.h>
#include <math.h>
using namespace std;
int cnt=0;
int a[100009][2];
void dfs(int node,int length){
    if(node==0){return;}
    cnt=fmax(cnt,length);
    dfs(a[node][0],length+1);
    dfs(a[node][1],length+1);

}
int main(){
    int n;
    scanf("%d",&n);
    
    for (int i=1;i<=n;i++){
        int ax = 0;int bx = 0;
        scanf("%d %d",&ax,&bx);
    a[i][0]=ax;
    a[i][1]=bx;
    }
    dfs(1,1);
    printf("%d\n",cnt);
}
