fn main() {
    //let haystack=String::from("sadbutsad");
    let haystack=String::from("mississippi");
    //let needle=String::from("byt");
    let needle=String::from("issip");

    let result=str_str(haystack,needle);
    println!("{}",result);
}

 pub fn str_str(haystack: String, needle: String) -> i32 {
    let mut index:i32=-1;
    let needle_len=needle.len();
    let mut in_needle=false;
    let mut count=0;
    let first_char=needle.chars().next().unwrap();
    let mut checknext=0;
    let mut ongoing=true;

    let mut needlechars=needle.chars();
    if needle_len==0 || haystack.len()==0 || needle_len>haystack.len(){
        return -1;
    }
    if needle==haystack{
        return 0;
    }
    

    while ongoing==true{
        let mut haystckchars=haystack.chars();
    
        let mut needlechars=needle.chars();
        //start iterating from checknext
        
        for (indx,char) in haystckchars.into_iter().skip(checknext as usize).enumerate(){

            println!("char: {}",char);
            println!("indx: {}",indx);

           if char==first_char{
               if in_needle==true{
                   checknext=indx.clone()-1;
               }
               
               if in_needle==false{
                index=indx.clone() as i32;
                println!("index: {}",index);
                in_needle=true;
               }
           }
           if in_needle==true{
            if char!=needlechars.next().unwrap(){
                index=-1;
                count=0;
                in_needle=false;
                needlechars=needle.chars();
                if checknext!=0{
                    //reset index to checknext
                    break;

                }
            }
            else{
                
                count+=1;
                println!("count: {}",count);
                if count==needle_len{
                    checknext=0;
                    break;
                }
            }
           }
        }
        if checknext==0{
            ongoing=false;
        }
        else{
            checknext=0;
        }

    }
    index 
}
