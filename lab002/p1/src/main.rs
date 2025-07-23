
fn add_chars_n(s:String,c:char,n:usize)->String{
    let mut new_string = s;
    for _ in 0..n{
        new_string+=format!("{}",c).as_str();
    }
    return  new_string;
}

fn main() {
    let mut s = String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + 'a' as u8) as char;
        s = add_chars_n(s, c, 26 - i);

        i += 1;
    }

    print!("{}", s);
}
