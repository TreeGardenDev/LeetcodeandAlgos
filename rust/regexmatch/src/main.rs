pub fn is_match(string: String, pattern: String) -> bool {
    let mut matched:Vec<bool>=vec![false; string.len() ];
    let mut char=char::from(0); 
    let mut works=true;
    let mut asterisk = false;
    println!("{:?}", matched);


    string.chars().enumerate().for_each(|(j, c)| {
        pattern.chars().enumerate().for_each(|(i, p)| {
            if !works {
                return;
            }

            if c == p || p == '.' {
                matched[i]=true;
            } else if p == '*' {
                println!("Here");
                asterisk = true;
                println!("Char: {}", char);

                if char == '.' || char == c {
                    matched[i]=true;
                    return;
                } else {
                    matched[i]=false;
                }
            } else {
                matched[i]=false;
                works=false;
            }
            char=p;
        })
    });
    
    for i in 0..string.len() {
        if !matched[i] {
            return false;
        }
    }
    return true;
    }

fn main() {
    let s = "aabb".to_string();
    let p = "a*bb".to_string();
    println!("{}", is_match(s, p));
}
