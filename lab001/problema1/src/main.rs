
fn prime(x: i32) -> bool {
    if x < 2 {
        return false;
    }
    if x == 2 {
        return true;
    }
    if x != 2 && x % 2 == 0 {
        return false;
    }
    let mut i: i32 = 2;
    while i*i<x {
        if x % i == 0{
            return false;
        }
        i=i+1;
    }
    return true;
}
fn lowerthen100(){
    let mut i:i32=0 as i32;
    while i <= 100 {
        if prime(i) == true {
            print!("{} ",i);
        }
        i+=1;
    }
}
fn main() {
    lowerthen100();
}
