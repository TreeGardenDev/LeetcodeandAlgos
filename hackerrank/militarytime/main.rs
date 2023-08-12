fn main() {
    let date=&"12:34:56AM";
    let result=timeConversion(date);
    println!("{} -> {}",date,result);
}

fn timeConversion(s:&str)->String{
    let mut time = String::new();
    let mut hour = s.chars().take(2).collect::<String>();
    let mut min = s.chars().skip(3).take(2).collect::<String>();
    let mut sec = s.chars().skip(6).take(2).collect::<String>();

    let mut am_pm=s.chars().skip(8).collect::<String>();
    if am_pm=="PM"{
        if hour!="12"{
            hour= (hour.parse::<i32>().unwrap()+12).to_string();
        }
    }
    else{
        if hour=="12"{
            hour="00".to_string();
        }
    }

    time=format!("{}:{}:{}",hour,min,sec);
    time
}
