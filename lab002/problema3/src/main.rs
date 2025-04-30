
fn add_space(s:&mut String,n :i32){
    let mut i:i32=0;
    while i < n {
        s.push(' ');
        i+=1;
    }
}
fn add_str(s:&mut String , s2: &str){
    for i in 0..s2.len(){
        s.push(s2.as_bytes()[i] as char);
    }
}
fn add_integer(s:&mut String ,mut n :i32){
    let mut n2:i32=0;
    while n > 0 {
        n2=n%10+n2*10;
        n/=10;
    }
    let mut i:i32=0;
    while n2 > 0 {
        s.push((n2%10+48) as u8 as char);
        i+=1;
        if i==3 {
            s.push('_');
            i=0;
        }
        n2/=10;
    }
    if i==0 {
        s.pop();
    }
}
fn add_float(s:&mut String , n:f32){
    s.push_str(&n.to_string());
}
fn main() {
   let mut s:String = String::from("");
   
   add_space(& mut s,40);
   add_str(&mut s,"I");
   add_space(& mut s,1);
   add_str(&mut s,"💚");
   add_str(&mut s ,"\n");
   add_space(& mut s,40);
   add_str(&mut s,"RUST.");
   add_space(& mut s,40);
   add_str(&mut s ,"\n");
   add_space(&mut s, 5);
   add_str(&mut s, "Most");
   add_space(&mut s, 12);
   add_str(&mut s, "create");
   add_space(&mut s, 5);
   add_integer(&mut s, 306437968);
   add_space(&mut s, 12);
   add_str(&mut s, "and");
   add_space(&mut s, 5);
   add_str(&mut s, "lastest");
   add_space(&mut s, 8);
   add_str(&mut s, "is\n");
   add_space(&mut s, 10);
   add_str(&mut s, "downloaded");
   add_space(&mut s, 8);
   add_str(&mut s, "has");
   add_space(&mut s, 14);
   add_str(&mut s, "downloads");
   add_space(&mut s, 5);
   add_str(&mut s, "the");
   add_space(&mut s, 9);
   add_str(&mut s, "version");
   add_space(&mut s, 4);
   add_float(&mut s, 2.038);
   print!("{}",s);
}
