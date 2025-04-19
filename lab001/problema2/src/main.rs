fn coprime(x1: i32, y1: i32) -> bool {
    
    let (mut x, mut y) = (x1, y1);
    if x1 == 0 || y1 == 0 {
        return false;
    }

    while x != y {
        if x > y {
            x -= y;
        } else {
            y -= x;
        }
    }

    x == 1
}

fn main() {
    let mut x :i32=0;
    while x <= 100 {
        let mut y :i32=0;
        while y <= 100 {
            println!("({}, {}): {}", x, y, coprime(x, y));
            y += 1;
        }
        x += 1;
    }
}
