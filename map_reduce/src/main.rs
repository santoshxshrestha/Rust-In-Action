#![allow(unused)]
use core::str;
use std::thread;
const MAX_THREADS: usize = 8;

fn main() {
    {
        let data = "
        59203486159064097517324897581947328610
        10864189600986950165896492368568913653
        52738049592346591005619030741397849837
        59203486159064097517324897581947328610
        10864189600986950165896492368568913653
        52738049592346591005619030741397849837
        59203486159064097517324897581947328610
        10864189600986950165896492368568913653
        52738049592346591005619030741397849837
        59203486159064097517324897581947328610
        21109701842816871408026310264028327883";


        let mut children = vec![];

        let chunked_data = data.split_whitespace();

        for (i, data_segment) in chunked_data.enumerate() {
            println!("data_segment {} is \"{}\"" , i , data_segment);

            children.push(thread::spawn(move || -> u32 {
                let result = data_segment
                    .chars()
                    // converting the text-characters to their number value ..
                    .map(|c| c.to_digit(10).expect("should be a digit"))
                    .sum();
                println!("processed segment {}, result={}", i , result);

                result

            }));
        }

        let final_result = children.into_iter().map(|c| c.join().unwrap()).sum::<u32>();

        println!("Final sum result: {}",final_result);


    }

    {
        let data = "
        59203486159064097517324897581947328610
        10864189600986950165896492368568913653
        52738049592346591005619030741397849837
        52738049592346591005619030741397849837
        59203486159064097517324897581947328610
        10864189600986950165896492368568913653
        52738049592346591005619030741397849837
        52738049592346591005619030741397849837
        59203486159064097517324897581947328610
        10864189600986950165896492368568913653
        52738049592346591005619030741397849837
        59203486159064097517324897581947328610
        10864189600986950165896492368568913653
        52738049592346591005619030741397849837
        59203486159064097517324897581947328610
        21109701842816871408026310264028327883";

        let lines_of_data: Vec<&str> = data.split_whitespace().collect();
        let total_lines = lines_of_data.len();
        let num_chunks = std::cmp::min(MAX_THREADS, total_lines);
        let chunk_size = (total_lines + num_chunks - 1) / num_chunks;

        let mut handles = Vec::new();

        for data_segment in lines_of_data.chunks(chunk_size) {
            let chunk = data_segment.to_vec(); // Create owned copy of the data segment
            let handle = thread::spawn(move || -> u128 {
                chunk.iter().map(|line| {
                    line
                        .chars()
                        .filter_map(|d| d.to_digit(10))
                        .map(|d| d as u128)
                        .sum::<u128>()
                }).sum()
            });
            handles.push(handle);
        }

        let final_result: u128 = handles.into_iter()
            .map(|handle| handle.join().unwrap_or(0))
            .sum();

        println!("Final sum result: {}", final_result);
    }
}
