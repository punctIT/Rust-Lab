//10
fn rabin_karp(t:&String,p:&String)->i32{
    
    fn pow(i:i32,p:usize,q:i32)->i32{
        let mut aux:i32=1;
        for _e in 0..p {
            aux*=i;
            aux%=q;
        }
        return aux;
    }
    let mut t_array: Vec<i32> = vec![0; t.len()];
    let mut p_array: Vec<i32> = vec![0; p.len()];
    fn make_arrays(array: &mut Vec<i32>,s: & String){
        let mut index:usize=0;
        for i in s.chars(){
            let mut nr=i as u8 as i32;
            if i >= 'a' && i<='z'{
                nr-=32;
            }
            nr-=65;
            array[index]=nr;
            index+=1;
        }
    }
    make_arrays(&mut t_array,&t);
    make_arrays(&mut p_array,&p);

    fn fq(array:&Vec<i32>,len:usize)->i32{
        let q = 23;
        let mut result=0;
        for i in 0..len{
            result=(result*26+ array[i])%q;
        }
        return result;
    }
    let x:i32 = fq(&p_array,p.len());
    let mut y:i32 = fq(&t_array,p.len());
    let q = 23;
    let m = p.len();
    for i in 0..=t.len()-m{
        if x == y {
            let mut end:bool =true;
            for _e in 0..m {
                if p_array[_e]!= t_array[_e+i] {
                    end=false;
                    break;
                } 
            }
            if end == true {
                return i as i32 ;
            }
        }
        if i < t.len() - m {
            y = ((y + q - (t_array[i] * pow(26, m - 1, q)) % q) * 26 + t_array[i + m]) % q;
            y = (y + q) % q;//  modulul poate fi negativ
        }
    }

    return -1;
}

//14
#[derive(Debug)]
enum ErrorType{
    MatrixSizeIncompatibleN,
    MatrixSizeIncompatibleM,
}

fn matrix_ainb(a:&Vec<Vec<i32>>,b:&Vec<Vec<i32>>)->Result<(i32,i32),ErrorType>{
    if a.len() > b.len(){
        return Err(ErrorType::MatrixSizeIncompatibleN);
    }
    if a[0].len() > b[0].len(){
        return Err(ErrorType::MatrixSizeIncompatibleM);
    }
    let (n1,n2,m1,m2):(usize,usize,usize,usize)
        =(a.len(),b.len(),a[0].len(),b[0].len());
    for i in 0..=n2-n1{
        for j in 0..=m2-m1{
            let mut exista:bool = true;
            'check: for x in 0..n1 {
                for y in 0..m1 {
                    if a[x][y] != b[i + x][j + y] {
                        exista = false;
                        break 'check;
                    }
                }
            }
            if exista == true {
                return Ok((i as i32 ,j as i32));
            }
        }
    }
    return Ok((-1,-1));
        
}
//15
fn matrix_rabin_karp(p:&Vec<Vec<char>>,t:&Vec<Vec<char>>)->Result<(i32,i32),ErrorType>{
    if p.len() > t.len(){
        return Err(ErrorType::MatrixSizeIncompatibleN);
    }
    if p[0].len() > t[0].len(){
        return Err(ErrorType::MatrixSizeIncompatibleM);
    }
    let mut t_matrix: Vec<Vec<i32>> = vec![vec![0;t[0].len()];t.len()];
    let mut p_matrix: Vec<Vec<i32>> = vec![vec![0;p[0].len()];p.len()];
    fn make_matrixs(matrix: &mut Vec<Vec<i32>>,s: &Vec<Vec<char>>){
        let mut index:usize=0;
        for i in 0..s.len(){
            for q in 0..s[0].len(){
                let mut nr= s[i][q] as u8 as i32;
                if s[i][q] >= 'a' && s[i][q]<='z'{
                    nr-=32;
                }
                matrix[i][q]=nr;
            }
        }
    }
    make_matrixs(& mut t_matrix,t);
    make_matrixs(& mut p_matrix,p);
    fn fq(matrix: & Vec<Vec<i32>>,n:usize,m:usize)->i32{
        let mut result:i32=0;
        let q=23;
        for i in 0..n{
            let mut presult:i32=0;
            for w in 0..m{
                presult=(presult*26+ matrix[i][w])%q;
            }
            result=(result+presult)%q;
        }
        return result;
    }

    let x=fq(&p_matrix,p.len(),p[0].len());
    let mut y = fq(&t_matrix,p.len(),p[0].len());

    println!("{} {} ,,,",x,y);
    for i in 0..=t.len()-p.len(){
        for j in 0..=t[0].len()-p[0].len(){
            let mut este:bool=true;
            if x==y{
                'iesire: for i2 in 0 .. p.len(){
                    for j2 in 0..p[0].len(){
                        if p[i2][j2] != t[i+i2][j+j2] {
                            este=false;
                            break 'iesire;
                        }
                    }
                }
            }
            if este == true{
                return Ok((i as i32 , j as i32));
            }
            

        }
    }

    return Ok((-1,-1));


}

fn main() {
    println!("rabin karp");
    let t:String=String::from("AEsABC");
    let p:String=String::from("AC");
    println!("{}",rabin_karp(&t,&p));



    println!("\n Matrice In matrice");
    let rows = 3;
    let cols = 4;
    let mut matrix = vec![vec![0; cols]; rows];
    let matrix2 = vec![vec![1; cols-2]; rows-1];
    matrix[1][1]=1;
    matrix[1][2]=1;
    matrix[2][1]=1;
    matrix[2][2]=1;
    println!("{:?}", matrix);
    println!("{:?}", matrix2);
    if matrix_ainb(&matrix,&matrix2).is_ok(){
        println!("{:?}",matrix_ainb(&matrix,&matrix2).unwrap());
    }
    else {
        println!("eroare");
    }
    if matrix_ainb(&matrix2,&matrix).is_ok(){
        println!("{:?}",matrix_ainb(&matrix2,&matrix).unwrap());
    }
    else {
        println!("eroare");
    }

    println!("\n Rabin Karp-Matrice In matrice");
    
    let mut matrixc = vec![vec!['A'; cols]; rows];
    let matrixc2 = vec![vec!['B'; cols-2]; rows-1];
    if matrix_rabin_karp(&matrixc2,&matrixc).is_ok(){
        println!("{:?}",matrix_rabin_karp(&matrixc2,&matrixc).unwrap());
    }
    else {
        println!("eroare");
    }
}
