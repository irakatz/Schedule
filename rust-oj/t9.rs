//用rust写栈与队列
struct Stack<T>{
    stack:Vec<T>,
    size:usize,
}
impl <T> Stack<T>{
    pub fn new() -> Self {
        Stack {
            stack: Vec::new(),
            size:0
        }
    }
    pub fn peek(&mut self)->&T{
         &self.stack[self.size-1]
    }
    pub fn size(&mut self)->usize{
        self.size
    }
    pub fn push(&mut self, num:T){
        self.stack.push(num);
        self.size+=1;
    }
    pub fn pop(&mut self){
        self.stack.pop();
        self.size-=1;
    }
}
struct Queue<T>{
    stack:Vec<T>,
    size:usize,
}
impl <T> Queue<T>{
    pub fn new() -> Self {
        Queue {
            stack: Vec::new(),
            size:0
        }
    }
    pub fn tail(&mut self)->&T{
        &self.stack[self.size-1]
    }
    pub fn front(&mut self)->&T{
        &self.stack[0]
    }
    pub fn size(&mut self)->usize{
        self.size
    }
    pub fn offer(&mut self, num:T){
        self.stack.push(num);
        self.size+=1;
    }
    pub fn remove(&mut self){
        self.stack.remove(0);
        self.size-=1;
    }
}
fn main(){
    let mut j=Stack::new();
    j.push(1);
    j.push(2);
    j.pop();
    println!("{}",j.peek());
    println!("{}",j.size());
    let mut k=Queue::new();
    k.offer(1.1);
    k.offer(2.2);
    k.offer(3.3);
    k.remove();
    println!("{}",k.front());
    println!("{}",k.tail());
    println!("{}",k.size());

}