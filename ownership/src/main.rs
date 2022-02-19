fn differentstring(){
    let mut  s = String::from("hello");
    //pushing onto string
    s.push_str("world");
    println!("{}",s)//prints hello world
}
fn movingvariables(){
    let mut s  = String::from("hello world");
    let y = s;// s now invalid
   // println!("{}",s); causes error s now invalidated and moved 
}
fn cloningvariables(){
    let mut s = String::from("hello world");
    let y = s.clone();
    println!("s {}, y {}",s,y);
}
fn addingnumber(first:u32,second:u32)->u32{
    first+second
}
fn greeting(greet:String)->String{
    greet
}
fn greetingtwo(greet:&String){
   // greet.push("world"); can prefrom due not being mutabel
   println!("{}",greet);

}
fn mutates(greet:&mut String){
    greet.push_str("world");


}


fn main() {
    differentstring();
    cloningvariables();
    let x =  10;
    let y = 200;
    println!("x {} y {}",x, y);
    let calling = addingnumber(x,y);
    println!("calling function adding number {}",calling);
    println!("x {} y {}",x,y); // fine due to being a copy  known at comp time
    let mut s  = String::from("hello");
    let hello  = greeting(s);
    //println!("s is no longer in scope due to ownerships {}",s);
    //the variable of s ownership is now moved to hello variable
    println!("{}",hello);
    let mut new_s  = String::from("hi there");
    //println!("{}",greetingtwo(new_s));
    //println!("{}",new_s); ownership moved
    //to overcome this pass by reference
    greetingtwo(&new_s);
    mutates(& mut new_s);

}
