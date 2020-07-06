//判断括号匹配

fn main(){
    let mut input=String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut newinput=input.trim().to_string();
    let mut v:Vec<char> = Vec::new();
    let mut ist=true;
    for c in newinput.chars(){
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
    println!("{}",ist);
    // for i in 0..v.len(){
    //     println!("{}",v[i]);
    // }
}

