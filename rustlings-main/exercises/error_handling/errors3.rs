// errors3.rs
// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?
// Execute `rustlings hint errors3` for hints!

//不确定我们是否要把cost>token视作error，如果这样就有多个error类型了，怎么做

use std::num::ParseIntError;

fn main() ->Result<String,ParseIntError>{
    let mut tokens = 100;
    let pretend_user_input = "8";

    let cost = total_cost(pretend_user_input)?;

    if cost > tokens {
        Ok("You can't afford that many!".to_string())
    } else {
        tokens -= cost;
        Ok(format!("You now have {} tokens.", tokens))
    }
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}
