fn bottle_of_tedi() {
    let mut bottels = 99;
    while bottels >= 1 {
        if bottels != 1 {
            if bottels != 0 {
                println!("{} bottles of tedi on the wall", bottels);
                println!("{} bottles of tedi", bottels);
            } else {
                println!("No bottles of tedi on the wall");
                println!("No bottles of tedi");
            }
        } else {
            println!("{} bottle of tedi on the wall", bottels);
            println!("{} bottle of tedi", bottels);
        }
        println!("Take one down, pass it around");
        bottels -= 1;
        if bottels != 1 {
            if bottels != 0 {
                println!("{} bottles of tedi on the wall", bottels);
            } else {
                println!("No bottles of tedi on the wall");
            }
        } else {
            println!("{} bottle of tedi on the wall", bottels);
        }
        println!()
    }
    println!("No bottles of tedi on the wall");
    println!("No bottles of tedi");
    println!("Got to the store , buy some more,");
    println!("99 bottles of tedi on the wall");
}

fn main() {
    bottle_of_tedi()
}
