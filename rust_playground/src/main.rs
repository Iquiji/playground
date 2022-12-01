use std::fs::*;

fn main() -> Result<(),Box<dyn std::error::Error>>{
    println!("Hello, world!");

    let file = read_to_string("aoc_1.data")?;
    let mut grouped: Vec<Vec<u64>> = vec![];
    let mut current_group: Vec<u64> = vec![];
    for line in file.lines(){
        if line.is_empty(){
            grouped.push(current_group);
            current_group = vec![]
        }
        else {
            current_group.push(line.parse()?)
        }

    }
    if !current_group.is_empty(){
        grouped.push(current_group);
        current_group = vec![]
    }

    let mut summed: Vec<u64> = grouped.iter().map(|elf| {
        elf.iter().sum()
    }).collect();



    let max_idx = summed.iter().enumerate().max_by_key(|(_, &value)| value).map(|(idx, _)| idx).unwrap();
    let mut max_val = summed[max_idx];
    summed.remove(max_idx);
    let max_idx = summed.iter().enumerate().max_by_key(|(_, &value)| value).map(|(idx, _)| idx).unwrap();
    max_val += summed[max_idx];
    summed.remove(max_idx);
    let max_idx = summed.iter().enumerate().max_by_key(|(_, &value)| value).map(|(idx, _)| idx).unwrap();
    max_val += summed[max_idx];
    summed.remove(max_idx);


    println!("{:?}",grouped);
    // println!("Max elf: {:?}", max_idx + 1);
    println!("Carrying: {:?}", max_val);

    Ok(())
}
