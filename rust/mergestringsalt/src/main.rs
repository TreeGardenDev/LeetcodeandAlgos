pub fn merge_alternately(word1: String, word2: String) -> String {
    let mut result = String::new();
    let wor1_len = word1.len();
    let wor2_len = word2.len();
    let diff = TryInto::<i32>::try_into(wor1_len).unwrap() - TryInto::<i32>::try_into(wor2_len).unwrap();
    let mut end=0;
    let mut string1larger=false;
    if diff>=0{
       end=wor2_len;
       string1larger=true;
    }else if diff<0 {
       end=wor1_len;
       string1larger=false;
       
    }
    for i in 0..end{
        String::push_str(&mut result, word1.chars().nth(i).unwrap().to_string().as_str());
        String::push_str(&mut result, word2.chars().nth(i).unwrap().to_string().as_str());
    }
    //let remcount:i32=TryInto::<i32>::try_into(diff).unwrap().abs();
    if diff!=0{
        let remcount=TryInto::<i32>::try_into(diff).unwrap().abs() as usize;
        let rem_word:String;
        if string1larger{
            rem_word=word1.chars().skip(end).take(remcount).collect::<String>();
        }
        else{
            let difference=wor2_len-wor1_len;
            rem_word=word2.chars().skip(end).take(difference).collect::<String>();
        }
        String::push_str(&mut result, rem_word.as_str());

    }
    //}

    result
    }
fn main() {
    let word1 = "abc".to_string();
    let word2 = "pqrzzz".to_string();
    let result = merge_alternately(word1, word2);
    println!("{}", result);
}
