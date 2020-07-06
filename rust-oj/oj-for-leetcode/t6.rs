//有效括号匹配
//https://leetcode-cn.com/problems/valid-parentheses/
//给定一个只包括 '('，')'，'{'，'}'，'['，']' 的字符串，判断字符串是否有效。
//rust
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut v:Vec<char> = Vec::new();
        let mut ist=true;
        for c in s.chars(){
            if c=='}'{
                if(v.len()==0){ist=false;break;}
                if v[v.len()-1 as usize]=='{'{
                    v.pop();
                }
                else{ist=false;break;}
            }
            else if c==')'{
                if(v.len()==0){ist=false;break;}
                if v[v.len()-1 as usize]=='('{
                    v.pop();
                }
                else{ist=false;break;}
            }
            else if c==']'{
                if(v.len()==0){ist=false;break;}
                if v[v.len()-1 as usize]=='['{
                    v.pop();
                }
                else{ist=false;break;}
            }
            else{v.push(c);}
        }
        if v.len()!=0{ist=false;}
        return ist;
    }
}
//非rust
// class Solution {
// public boolean isValid(String s) {
// Stack<Integer>a=new Stack<>();
// for(int i=0;i<s.length();i++){
// char c=s.charAt(i);
// if(c=='('){a.push(1);}
// else if(c=='[')a.push(2);
// else if(c=='{')a.push(3);
// else if(c==')'){if(a.size()==0)return false;if(a.peek()==1)a.pop();else return false;}
// else if(c==']'){if(a.size()==0)return false;if(a.peek()==2)a.pop();else return false;}
// else if(c=='}'){if(a.size()==0)return false;if(a.peek()==3)a.pop();else return false;}
//
// }
// if(!a.isEmpty())return false;
// return true;
// }
// }