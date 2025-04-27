#![allow(unused)]

use std::vec;
#[derive(PartialEq,Eq)]
struct Point{
    x: i32,
    y: i32,
}

fn main() {
    let origin = Point{
        x: 0,

        y: 0
    };

    let compare: bool = origin.x==origin.y;
    println!("It is {compare} that they are equal.");
    println!("It is {} that they are equal.",origin.x==origin.y);

    {

        struct Student {
            name: String,
            class: u32,
            roll_no: u32,
        }

        let mut students:Vec<Student> = Vec::new();

        let harry = Student{
            name: "Harry".to_owned(),
            class: 9,
            roll_no: 2
        };
        students.push(harry);

        let strike = Student{
            name: "Strike".to_owned(),
            class: 10,
            roll_no: 4,
        };
        students.push(strike);

        let vinod = Student{
            name: "Vinod".to_owned(),
            class: 10,
            roll_no: 7,
        };
        students.push(vinod);

        let leonard = Student{
            name: "leonard".to_owned(),
            class: 10,
            roll_no: 7,
        };
        students.push(leonard);

        let mikasha = Student{
            name: "Mikasha".to_owned(),
            class: 10,
            roll_no: 4,
        };
        students.push(mikasha);

        let santosh = Student{
            name: "Santosh".to_owned(),
            class: 14,
            roll_no: 49,
        };
        students.push(santosh);

        println!();
        for student in students.iter() {
            let mut has_proxy = false;

            for other_student in students.iter(){
                if student.class  == other_student.class
                    && student.roll_no == other_student.roll_no
                        && student.name != other_student.name{
                            println!("There is a proxy of {} and his name is {}",student.name,other_student.name);
                            println!("He/she is in tha class {}", other_student.class);
                            println!("and, the roll number is {}",other_student.roll_no);
                            println!();
                            has_proxy = true;
                }
                if !has_proxy{
                    println!("The name of the student is {}",student.name);
                    println!("His/her class is {}",student.class);
                    println!("and, his roll number is {}",student.roll_no);

                    println!();
                }

            }
        }

    }
}
