pub fn convert(s: String, num_rows: i32) -> String {
    let mut result = String::new();
    let mut row = 0;
    let mut down = true;
    let mut rows: Vec<String> = Vec::new();
    if num_rows == 1 {
        return s;
    }
    for _ in 0..num_rows {
        rows.push(String::new());
    }
    for c in s.chars() {
        rows[row].push(c);
        if down {
            row += 1;
        } else {
            row -= 1;
        }
        if row == num_rows as usize - 1 {
            down = false;
        } else if row == 0 {
            down = true;
        }
    }
    for r in rows {
        result.push_str(&r);
    }
    result
        
}

fn main() {
    //let s = String::from("PAYPALISHIRING");
//    let s=String::from("AB");

    let s=String::from("orfefwofihmozumjkgeilldtosfnorocltisrxxbelrdhdormtaxeftuxyhosxsdtbxkyuoehfkolyfxthwympskqcaibsnutkhtevylciznigkcohccywooaoychlycvfwbcuofepuowfqmouoordfttdvapudkbzmgvclzsfpomiaccqtvvyppdmrsiufkvtqqvdrnkjbzrddtwwrtwiiaucsdwzpushmomgdyphxgmdbibucycmyxoscnutjmcvcqdgoupocbremuaqsdcsctneihzrvboyrsghmvvpyovkjvadadwcylggshzninnbhvjusglrvibgdejgjfihqrpkyoajdpkllvhfeswzaahfeqlnyuwqnlblbdwesjpdewjiohjqjqynjlchhyxulagmdcrwlgbsfmcvwomfgmtpxxyfywzjyhycmpyxxbrcowakkmpqakixkgciectdjrhvghvgiokykkkuycnymvwydagicanorwladiilxsmhfwedytenocltcsdfusvnognrrvfoqrxvpdyowedmgoijilqeelsstfmkdtatkaobforctuqbjyktmayqnqkhxytarwvdyjfdawhvrywcyhxkjvcxnpglnbnfxjkxspbuoiphimjhvgteewbrnhcajqhibugtjjqzrfgctploygteewvrgaupsbztxhohqegkmpmfezuefpiklgbrgviazktwrjfiooucdihjhdqosayegcxozgoaqjzjtgtjunlzvuleydvqdtwkxuazcpzuaafthzedorfmmqqktlcyhbigvjfzahvahawozcsouxaipsukgwipztvuebvgiqgpregqhagdzilobfajdiyddtzhwvpgnwyecexlgfofozvrgvamfarlvsuspkydiyjkegwkokpcmkvuhvipvzaquwkjglmojyzogxyuhqwvctsmoadlcfewbqfibuwnuxdaudvevtbyntmdup");
    println!("{}", s.len());
    let num_rows = 620;
    let result = convert(s, num_rows);
    println!("{}", result);
}
pub fn convert2(s: String, num_rows: i32) -> String {
    let mut result = String::new();
    if num_rows == 1 {
        return s;
    }
    if s.len() == 0 {
        return result;
    }
    if num_rows>=s.len() as i32{
        return s;
    }
    let matched=false;

    while !matched{
        
        
        
    }
    



    result
        
}
