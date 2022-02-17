fn degreestofran(degrees:u32)-> u32{
    let mut div =  degrees / 5;
    div = div * 9;
     div + 32


}


fn main() {
    let convert = degreestofran(25);
    println!("Degrees to franherheit {}",convert);
}
