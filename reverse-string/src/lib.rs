pub fn reverse(input: &str) -> String {
    let ret: String = input.chars().rev().collect();
    return ret
}

pub fn main(){
    let input : &str = "uüu";
    let ret = reverse(input);
    println!("{}", ret);
}

