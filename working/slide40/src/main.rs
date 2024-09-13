fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = x.clone();    //  |       |
    }                     // -+       |
                          //          |
    println!("{}", r);    //          |
}