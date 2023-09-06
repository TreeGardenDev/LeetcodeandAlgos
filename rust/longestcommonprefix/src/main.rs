pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let arraylen = strs.len();
    let mut maxlen = strs[0].len();
    for i in 0..strs.len() - 1 {
        let mut currentlen = 0;
        for j in 0..strs[i].len() {
            if i + 1 < arraylen && strs[i].chars().nth(j) == strs[i + 1].chars().nth(j) {
                currentlen += 1;
            } else {
                if maxlen > currentlen {
                    maxlen = currentlen;
                }
                break;
            }
        }
    }
    let result = strs[0].chars().take(maxlen).collect::<String>();

    result
}
fn main() {
    let strs = vec![
        String::from("flower"),
        String::from("flow"),
        String::from("flight"),
    ];
    let result = longest_common_prefix(strs);
    println!("{}", result);
}
