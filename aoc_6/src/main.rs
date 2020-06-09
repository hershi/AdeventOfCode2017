use std::collections::HashMap;

fn next_step(banks : &mut Vec<i32>) {
    let blocks = *banks.iter().max().unwrap();
    let inc = blocks / banks.len() as i32;
    let mut extra = blocks % banks.len() as i32;

    {
        let mut current = banks.iter_mut().skip_while(|x| **x != blocks);
        *(current.next().unwrap()) = 0;
        current.take(extra as usize).for_each(|x| { *x += 1; extra -= 1; });
    }

    banks.iter_mut().take(extra as usize).for_each(|x| { *x += 1; extra -= 1; });

    banks.iter_mut().for_each(|x| *x += inc);
    //println!("inc {} extra {} result {}", inc, extra, blocks);
}

fn main() {
    //let mut banks = vec![0,2,7,0];
    let mut banks = vec![2,8,8,5,4,2,3,1,5,5,1,2,15,13,5,14];
    let mut seen = HashMap::new();
    let mut count = 0;
    println!("   {:?} ", banks);
    while !seen.contains_key(&banks) {
        seen.insert(banks.clone(), count);
        count += 1;
        next_step(&mut banks);
        //println!("   {:?} ", banks);
    }

    //println!("{:?}", banks);
    //println!("{:?}", seen);

    //println!("{}", *seen.entry(banks.clone()).or_default());
    println!("{} steps {}", count, count - *seen.entry(banks.clone()).or_default());
}
