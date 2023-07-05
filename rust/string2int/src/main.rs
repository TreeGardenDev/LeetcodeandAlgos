pub fn my_atoi(s: String) -> i32 {
    //FUCK THIS GODDAMN QUESTION
    let mut result = 0;
    let mut is_positive = false;
    let mut is_negative = false;
    let mut received_digit = false;
    let mut ignore = false;
    let mut is_max = false;
    let mut is_min = false;
    s.chars().for_each(|c| {
        
        println!("c = {}", c);
        if ignore || is_max ||is_min {
            return;
        }

        if !c.is_digit(10) && c != '-' && c != '+' && c != ' '{ 


            ignore = true;
            return;
        }
        else if c.is_digit(10) {
        received_digit = true;

        println!("I32 min + result= {}", i32::MIN+ result );
        println!("I32 max - result= {}", i32::MAX - result);
        println!("I32 min/10 + result = {}", (i32::MIN/10)  + result+9);
        println!("I32 max/10 - result = {}", (i32::MAX/10) - result+9);
        println!("C = {}", c.to_digit(10).unwrap() as i32);
        let maxcheck = (i32::MAX/10) - (result);
        let mincheck = (i32::MIN/10) + (result);
        println!("maxcheck = {}", maxcheck);
        if !is_negative&&maxcheck   < c.to_digit(10).unwrap() as i32 && c.to_digit(10).unwrap() as i32 > 7 {


            println!("maccheck = {}", maxcheck);
            println!("c = {}", c.to_digit(10).unwrap() as i32);
            println!("Hit max");
            is_max = true;
            return;
        }
        
        else if  is_negative && mincheck > -(c.to_digit(10).unwrap()  as i32) +9 {
            println!("Hit min");
            is_min = true;
            return;
        }
        
        result = result * 10 + c.to_digit(10).unwrap() as i32;

        }
        else if c == '-' {
            if received_digit {
                ignore = true;
                return;
            }
            is_negative = true;
        }
        else if c == '+' {
            if received_digit {
                ignore = true;
                return;
            }
            is_positive= true;
        }
        else if c == ' ' {
            if received_digit {
                ignore = true;
                return;
            }
        }
        if is_positive && is_negative {
            ignore = true;
            return;
        }
        
    });
    if is_negative {
        result = -result;
    }
    if is_max {
        result = i32::MAX;
        if is_negative {
            result=i32::MIN;
        }
    }
    if is_min {
        result = i32::MIN;
    }
    result
    
}
fn main() {
    let s = "21474836460".to_string();
    //let s = " -42".to_string();
    let result = my_atoi(s);
    println!("result = {}", result);
}
