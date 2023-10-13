pub fn start(limit: i128) {
    let mut f1 = 0;
    let mut f2 = 1;
    while f2 <= limit {
        if f1 == 0 && f2 == 1 {
            println!("{}", f1);
            println!("{}", f2);
        }
        let temp = f2;
        f2 = f2 + f1;
        f1 = temp;
        println!("{}", f2);
    }
}
