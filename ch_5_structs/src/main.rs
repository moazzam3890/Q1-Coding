use std::io;

#[derive(Debug)] 
struct Person { //Made a Blue print
    id : u8,
    first_name : String,
    last_name : String,
    age : u8,
    height : f32,
}

impl Person {
    //Associated function has been defined:
    fn data(id: u8, first_name:String, last_name:String, age:u8, height:f32) -> Person {
        Person {
            id,
            first_name,
            last_name,
            age,
            height
        }
    }
    //Method has been defined that printing of complete details will perform whenever its called:
    fn printing_whole(&self){
        println!("Complete details are : {:#?}", self);
    }
}

fn main () {
    println!("Please Enter your ID");
    let mut id_in = String::new();
    io::stdin().read_line(&mut id_in).expect("Please Enter a Valid ID");
    let id_in: u8 = id_in.trim().parse().expect("Please enter a Valid ID");

    println!("Please Enter your First Name");
    let mut f_name_in = String::new();
    io::stdin().read_line(&mut f_name_in).expect("Please Enter a Valid Name");

    println!("Please Enter your Last Name");
    let mut l_name_in = String::new();
    io::stdin().read_line(&mut l_name_in).expect("Please Enter a Valid Name");

    let mut age_in = String::new();
    io::stdin().read_line(&mut age_in).expect("Please Enter a Valid Age");
    let age_in: u8 = age_in.trim().parse().expect("Please enter a Valid Age");

    let mut height_in = String::new();
    io::stdin().read_line(&mut height_in).expect("Please Enter a Valid Height");
    let height_in: f32 = height_in.trim().parse().expect("Please enter a Valid Height");

    //calling Associated Function from here:
    let person = Person::data(id_in, f_name_in, l_name_in, age_in, height_in);

    //Calling Method:
    person.printing_whole();

}
