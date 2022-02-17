fn fibnumber(n:u32)->u32{
    let mut a  =0;
    let mut b  = 1;
    let mut c  = 0;
    if n == 0{
       return a;
    }
    for x in(2..n+1){
        c = a +b;
        a  = b;
        b = c;
    }
    b
}
fn main() {
    let store = fibnumber(10);
    println!("fib number {}",store);
}
