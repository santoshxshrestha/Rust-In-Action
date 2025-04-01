use std::vec;

fn main() {
    let vector1 = vec![3,4,52,52,3];
    let vector2 = vec![31,41,2,1,41,21,4131,2];
    println!("52 in vecter1:{}",vector1.iter().any(|&x| x==52) );
    //here difference between the iter() and into_iter() is also shown 
    //iter() converts the value into the refrence but the into_iter consumes 
    //the value so we get to use the value we do not have to do the destruction
    println!("9 in vecter1:{}",vector2.into_iter().any(|x| x==9) );

    //here we can access the vector1 because we have not transfered the ownership of the vector
    println!("the last element of the vector1 is {}",vector1[vector1.len()-1]);

    //we cant access the vector2 because hte into_iter have consued the vector2
    // println!("the first element of the vector2 is {} ",vector2[0]);
    {
        let another_vec1 = vec![12,41,21,41,3,41,3,141,31];
        let another_vec2 = vec![21,31,1,2,41,31,2,1,2,41,2,41,2,3];

        println!("find 31 in the another_vec1 {:?}",another_vec1.into_iter().find(|&x|x==31));

        //the previous thing is applied here too
        println!("find 31 in the another_vec2 {:?}",another_vec2.iter().find(|&&x|x==31));
    }

    //lets do the same thing in the case of the array 
    {
        let  array1 = [21,12,21,31,2,1,2,1,4,1,12];
        let  array2 = [31,41,41,31,1,2,41,41,31,];
        println!("find 12 in the array1 {:?}",array1.iter().find(|&&x|x==12));

        let array3 = &array2;
        println!("find 12 in the array3 {:?}",array3.into_iter().find(|&&x|x==12));

    }


    //iterator position will give the index not the refrence 
    println!("printing the position");
    {

        //position does not takes the refrence so we have to derefrence the vals
        let  array1 = [21,12,21,31,2,1,2,1,4,1,12];
        let  array2 = [31,41,41,31,1,2,41,41,31,];
        println!("find 12 in the array1 {:?}",array1.iter().position(|&x|x==12));
        println!("find the first even number in the array1 {:?}",array1.iter().position(|&x| x % 2 == 0));

        let array3 = &array2;
        println!("find 12 in the array3 {:?}",array3.into_iter().position(|&x|x==12));

    }
}
