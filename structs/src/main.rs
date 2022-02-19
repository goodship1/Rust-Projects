struct User {
    name: String,
    email:String,
    sign_in_count:u32,
    active:bool
}

struct Rectangle{
    length:u32,
    width:u32
}

fn builduser(name:String,email:String)-> User {
     User {
        name:name,
        email:email,
        sign_in_count: 100,
        active:true
    }
}

fn area(length:u32,width:u32)->u32{
    length * width
}


fn main() {
   let rect_one = Rectangle{ length:100,width:300};
   let store_area = area(rect_one.length,rect_one.width);
   println!("area of {}",store_area);

}
