//small program using structs student record program
struct Record{
    name:String,
    address:String,
    age:u32,
    grade:u32
}

impl Record{
    fn comparegrades(&self,other:&Record)->bool{
        self.grade < other.grade
    }
}
fn addingstudents(grade_one:u32,grade_two:u32)->(Record,Record){
    let first_grade:u32 = grade_one;
    let second_grade:u32 = grade_two;
    let student_one = Record{name:"Gavin".to_string(),address:"the moon".to_string(),age:3000,grade:first_grade};
    let student_two = Record{name:"lee".to_string(),address:"the sun".to_string(),age:3400,grade:second_grade};
    (student_one,student_two)
}


fn main() {
    let mut honor: Vec<Record> = Vec::new();
    let first_compare = addingstudents(100,90);
    let second_compare = addingstudents(200,50);
    if first_compare.0.comparegrades(&first_compare.1){
            honor.push(first_compare.0);
        }
    }

