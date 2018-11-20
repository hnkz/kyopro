fn main() {
    let modnum = 1000000007u64;
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let s: Vec<char> = s.chars().collect();

    let mut q_count = 0u64;
    let mut pow = 1u64;
    let mut pre = 0u64;
    let mut one_count = 0u64;
    let mut total_count = 0u64;
    for c in s.iter() {
        match  *c {
            '?' => {
                // 0と1に分岐するため、今までのトータルを足して、0の場合に今までの１の数分足している。
                total_count = (total_count * 2 % modnum) + (one_count * pow % modnum); 
                total_count %= modnum;
                // 0の場合、今までの?が1になる回数分だけ、足す
                total_count += q_count * pre % modnum;
                total_count %= modnum;
            
                pre = pow;
                pow *= 2;
                pow %= modnum;
                q_count += 1;
            }
            '0' => {
                if q_count > 0 {
                    total_count += (one_count * pow % modnum) + (q_count * pre % modnum);
                    total_count %= modnum;
                } else {
                    total_count += one_count;
                    total_count %= modnum;
                }
            }
            '1' => {
                one_count += 1;
                one_count %= modnum;
            }
            _ => {}
        }
    }

    println!("{}", total_count % modnum);
    
}
