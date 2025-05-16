fn is_valid_sudoku(b: &Vec<Vec<char>>,k:bool) -> bool { // k=true - verifica daca e termiant , si corect , k=false ,verifica daca tabela sudoku e rezolvabila (corecta)
    for i in 0..3{
        for j in 0..3{
            let mut v:Vec<i32>=vec![0;10];
            for w in 0..3{
                for e in 0..3{
                        if b[3*i+w][e+j*3] == '.' && k==false{
                            continue;
                        }
                        if b[3*i+w][e+j*3] > '9' || b[3*i+w][e+j*3] < '0' {
                             return false;
                        }

                        v[b[3*i+w][e+j*3] as u8 as usize -48] +=1;
                    }
                }
                for w in v{
                    if w > 1 {
                        return false;
                    }
                }
            }
        }
        for i in 0..9{
             let mut v:Vec<i32>=vec![0;10];
             for q in 0..9{
                if b[i][q]=='.'{
                    continue;
                }
                v[b[i][q] as u8 as usize -48]+=1;
             }
             for w in v{
                    if w > 1 {
                        return false;
                    }
                }
        }
        for i in 0..9{
             let mut v:Vec<i32>=vec![0;10];
             for q in 0..9{
                 if b[q][i]=='.'{
                    continue;
                }
                v[b[q][i] as u8 as usize -48]+=1;
             }
             for w in v{
                    if w > 1 {
                        return false;
                    }
                }
        }
    return true;
}


fn print_sudoku(m:& Vec<Vec<char>>){
     println!("");
    for i in 0..m.len(){
        for w in 0..m[0].len(){
            if w%3==0{
                print!("|");
            }
            print!("{} ",m[i][w]);
            if w==m[0].len()-1{
                 print!("|");
            }
        }
       
        if (i+1)%3==0 && i != m.len()-1{
            println!("");
            for _w in 0..m[0].len()*2+4{
                print!("-");
        }
        }
       
        println!("");
    }
}

fn solve_sudoku(board: &mut Vec<Vec<char>>) -> bool {
  

    for i in 0..9 {
        for j in 0..9 {
            if board[i][j] == '.' {
                for k in 0..=9 {
                   
                    let k:u8=  k + 48;
                    board[i][j] = k as char;
                    if is_valid_sudoku(board, false) {
                        if solve_sudoku(board) {
                            return true;
                    }
                    }
                   
                    board[i][j] = '.';
                }
                return false; 
            }
        }
    }
    return is_valid_sudoku(board, true);
}

fn sub_set_sum(v:&mut Vec<i32>,sum:i32)->Vec<Vec<i32>>{
    let mut out:Vec<Vec<i32>>=Vec::new();
     let mut current: Vec<i32> = Vec::new();
     v.sort();
     fn backtrack(v: &Vec<i32>, sum: i32, k: usize, current: &mut Vec<i32>, out: &mut Vec<Vec<i32>>, current_sum: i32){
        if current_sum == sum {
            out.push(current.clone());
        }
        if k >= v.len() || current_sum > sum {
            return;
        }
         for i in k..v.len() {
            if i != k && v[i] == v[i - 1] {
                continue;
            }

            current.push(v[i]);
            backtrack(v, sum, i + 1, current, out, current_sum + v[i]);
            current.pop();
        }

    }
    backtrack(v, sum, 0, &mut current, &mut out, 0);
    return out;
}

fn main() {
    println!("Sudoku");
     let mut m:Vec<Vec<char>> = vec![
        vec!['5','3','.','.','7','.','.','.','.'],
        vec!['6','.','.','1','9','5','.','.','.'],
        vec!['.','9','8','.','.','.','.','6','2'],
        vec!['8','.','.','.','6','.','.','.','3'],
        vec!['4','.','.','8','.','3','.','.','1'],
        vec!['7','.','.','.','2','.','.','.','6'],
        vec!['.','6','.','.','.','.','2','8','.'],
        vec!['.','.','.','4','1','9','.','.','5'],
        vec!['.','.','.','.','8','.','.','7','9'],
    ];

    println!("Tabla inițială:");
    print_sudoku(&m);

    if !is_valid_sudoku(&m, false) {
        println!("Eroare: tabla inițială NU este validă.");
    } else if solve_sudoku(&mut m) {
        println!("\nRezolvare găsită:");
        print_sudoku(&m);
    } else {
        println!("Nu s-a putut rezolva.");
    }
    println!("Subsetsum");
    let mut v:Vec<i32>=vec![1,2,4,-2,5,7];
    let sum=5;
    let v_out:Vec<Vec<i32>>=sub_set_sum(&mut v,sum);
    println!("{:?}",v_out);
}
