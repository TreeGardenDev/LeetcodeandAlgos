use std::collections::HashMap;

fn main() {
    let num = 3;
    let roman = int_to_roman(num);
    println!("{} = {}", num, roman);
}

pub fn int_to_roman(num: i32) -> String {

    let mut map=HashMap::new();
    map.insert(1, "I");
    map.insert(4, "IV");
    map.insert(5, "V");
    map.insert(9, "IX");
    map.insert(10, "X");
    map.insert(40, "XL");
    map.insert(50, "L");
    map.insert(90, "XC");
    map.insert(100, "C");
    map.insert(400, "CD");
    map.insert(500, "D");
    map.insert(900, "CM");
    map.insert(1000, "M");
    println!("{:?}", map);

    let mut remaining=num;
    let mut result=String::new();

    while remaining>0{
        //get the highest key that is less than or equal to remaining
        let key=map.keys().filter(|&x| x<=&remaining).max().unwrap();

        result.push_str(map.get(key).unwrap());
        remaining-=key;
    }

    result
        
}
