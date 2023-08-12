fn main() {
    //0 2 5 3
    let x1 = 0;
    let v1 = 2;
    let x2 = 5;
    let v2 = 3;

    let result = kangaroo(x1, v1, x2, v2);
    println!("{}", result);
}

fn kangaroo(x1:i32, v1:i32, x2:i32, v2:i32)->String{
    let kang1=(x1,v1);
    let kang2=(x2,v2);
    
    let mut result="NO".to_string();

    let lower=if x1<x2 {kang1} else {kang2};
    let higher=if x1>x2 {kang1} else {kang2};
    if lower.1<higher.1{
        return result;
    }
    let mut lower_pos=lower.0;
    let mut higher_pos=higher.0;
    loop{
        lower_pos+=lower.1;
        higher_pos+=higher.1;
        if lower_pos==higher_pos{
            result="YES".to_string();
            break;
        }
        if lower_pos>higher_pos{
            break;
        }
    }
    result


}
