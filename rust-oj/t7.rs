//排序算法实现
//还没有实现快排和归并排序
fn selectsort(inputvec:Vec<i32>) -> Vec<i32> {
    let mut prevvec =inputvec.clone();
    let mut v:Vec<i32> = Vec::new();
    while !prevvec.is_empty() {
        let mut minid=0;
        let mut minnum=prevvec[0];
        for i in 0..prevvec.len(){
            if prevvec[i]<minnum{minid=i;minnum=prevvec[i];}
        }
        prevvec.remove(minid);
        v.push(minnum);
    }
    return v;
}
fn bubblesort(inputvec:Vec<i32>) -> Vec<i32> {
    let mut v =inputvec.clone();
    while true{
        let mut ist=true;
        for i in 0..inputvec.len()-1{
            if v[i]>v[i+1]{
                ist=false;
                let mut tmp=v[i];
                v[i]=v[i+1];
                v[i+1]=tmp;
            }
        }
        if ist{break;}
    }
    return v;
}
fn insertsort(inputvec:Vec<i32>) -> Vec<i32> {
    let mut prevvec =inputvec.clone();
    let mut v:Vec<i32> = Vec::new();
    for i in 0..prevvec.len(){
        let mut id=v.len();
        for j in 0..v.len(){
            if prevvec[i]<v[j]{id=j;break;}
        }
        v.insert(id,prevvec[i]);
    }
    return v;
}
fn main() {
    let mut a:Vec<i32> = Vec::new();
    a.push(2);
    a.push(5);
    a.push(4);
    a.push(1);
    a.push(3);
    // a=selectsort(a);
    // a=bubblesort(a);
    a=insertsort(a);
    for i in 0..5{
        print!("{}",a[i as usize])
    }
}