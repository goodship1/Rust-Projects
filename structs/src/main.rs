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

impl Rectangle {
    fn area(&self)->u32{
        self.length * self.width
    }
    fn can_hold(&self,other:Rectangle)->bool{
        self.width < self.length && other.width < other.length
    }
}

fn builduser(name:String,email:String)-> User {
     User {
        name:name,
        email:email,
        sign_in_count: 100,
        active:true
    }
}
fn comparerectangles()->bool{
    let rect_one = Rectangle{length:100,width:200};
    let rect_two = Rectangle{length:200,width:200};
    let compare:bool  = rect_one.can_hold(rect_two);
    compare
}


//fn area(length:u32,width:u32)->u32{
   // length * width
//}


fn main() {
   let rect_one = Rectangle{ length:100,width:300};
   //let store_area = area(rect_one.length,rect_one.width);
   println!("area of {}",rect_one.area());
   comparerectangles;

}
