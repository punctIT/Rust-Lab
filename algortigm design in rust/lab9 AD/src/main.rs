
fn navie(x:&String,y:&String)->i32{
    let (n, m): (usize, usize) = (x.len(), y.len());
    let mut nrcom:i32=0;
    for i in 0..=(n - m) {
        let mut found:bool=true;
        for k in 0..m{
            nrcom+=1;
            if x.as_bytes()[i + k] != y.as_bytes()[k] {
                found = false;
                break;
            }
        }
        if found{
            let i = i as i32;
            println!("nr comparatii {} ",nrcom);
            return i
        }
    }
    println!("nr comparatii {} ",nrcom);
    return -1
}

fn kmp(t : &String , p : &String )->i32{

    fn prefix(f: &mut Vec<i32>, p: &String) {
        let m = p.len();
        let p_bytes = p.as_bytes();
        f[0] = -1;
        for i in 1..m {
            let mut k = f[i - 1];
            while k >= 0 && p_bytes[k as usize] != p_bytes[i - 1] {
                k = f[k as usize];
            }
            if k == -1 {
                f[i] = 0;
            } else {
                f[i] = k + 1;
            }
        }
    }

    let t_bytes = t.as_bytes();
    let p_bytes = p.as_bytes();

    let m = p.len();
    let n = t.len();

    let mut f: Vec<i32> = vec![0; m];

    prefix(&mut f,&p);
   
    let mut k=0;
    let mut i:usize=0;

    while i<=n-m && k < m as i32{
        if t_bytes[i+k as usize]==p_bytes[k as usize ]{
            k+=1;
        }
        else if k==0{
            i+=1;
        }
        else {
            i =i+k as usize -f[k as usize] as usize ;
            k=f[k as usize];
        }
        
    }
    if k == m as i32{
        return i as i32
    }
    -1
}
fn apariti_kmp(t : &String , p : &String )->i32{

    fn prefix(f: &mut Vec<i32>, p: &String) {
        let m = p.len();
        let p_bytes = p.as_bytes();
        f[0] = -1;
        for i in 1..m {
            let mut k = f[i - 1];
            while k >= 0 && p_bytes[k as usize] != p_bytes[i - 1] {
                k = f[k as usize];
            }
            if k == -1 {
                f[i] = 0;
            } else {
                f[i] = k + 1;
            }
        }
        for i in f{
            print!("{} ",i);
        }
        print!("\n");
    }

    let t_bytes = t.as_bytes();
    let p_bytes = p.as_bytes();

    let m = p.len();
    let n = t.len();

    let mut f: Vec<i32> = vec![0; m];

    prefix(&mut f,&p);
   
    let mut k=0;
    let mut i:usize=0;
    let mut apariti=0;

    while i<=n-m{
        if t_bytes[i+k as usize]==p_bytes[k as usize ]{
            k+=1;
            if k == m as i32{
                apariti+=1;
                i =i+k as usize -f[k as usize-1] as usize ;
                k=f[k as usize-1];
             }
        }
        else if k==0{
            i+=1;
        }
        else {
            i =i+k as usize -f[k as usize] as usize ;
            k=f[k as usize];
        }
        
    }
    apariti
}
fn main() {
    let s: String = String::from("BAAA");
    let sub:String = String::from("AAB");
    let sub2:String = String::from("CAA");
    println!("naiv {}, kmp {} ", navie(&s, &sub),kmp(&s,&sub));
    println!("naiv {}, kmp {} ", navie(&s, &sub2),kmp(&s,&sub2));
    print!("{}", apariti_kmp(&s,&sub));

}
