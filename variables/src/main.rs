fn adding(one:u32,two:u32)-> u32{
    one + two
}
fn greater(first:u32,second:u32)->bool{
    if first < second{
        true
    }
    else{
        false
    }
}
fn multicompare(first:u32)-> bool{
    if first > 100{
        true
    }else if first < 1000{
        true
    }
    else{
        false
    }
}
fn loopdemo(){
    loop{
        println!{"hello world"};
    }
}
fn whileloop(){
    let mut  number =  5;
    while number < 10{
        println!("this is number {}",number);
        number =  number+1;
    }
}
fn arrayloop(){
    let a  = [1,3,4,5];
    let mut x  = 0;
    while x < 4 {
        println!("this is index {}",a[x]);
        x =  x + 1;
    }
}
fn forlooparray(){
    let x  = [1,2,3,4,5,6];
    for elements in x.iter(){
        println!("{}",elements);
    }
}
fn rangeloop(){
    for x in(1..10){
        println!("{}",x);
    }
}
fn main() {
    let mut  x = 5;
    println!("this is x {}",x);
    let x =  100;
    println!("this is change of x {}",x);
    const MAX : u32 = 10000;
    //shadowing variables
    let y = 10;
    let y = y + 10;
    let y = y *10;
    let spaces = "  ";
    let spaces =  spaces.len();
    println!("shadowing spaces {}",spaces);
    let guess : u32 = "43".parse().expect("Not Number");
    println!("{}",guess);
    let store :(i32,f64,u8) = (500,6.4,1);
    let (a,b,c) = store;
    println!("tuple {}",a);
    let tup:(u32,f64,u8) =(1,1.0,2);
    println!("{}",tup.0);
    let c = [1,2,3,4];
    println!("{}",c[0]);
    let call = adding(10,10);
    println!("{}",call);
    let s:bool = greater(100,1000);
    println!("{}",s);
    let f:bool = multicompare(10);
    println!("{}",f);
    //loopdemo();
    arrayloop();
    forlooparray();

    

}
