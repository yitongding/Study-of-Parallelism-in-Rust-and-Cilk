extern crate rayon;
extern crate time;

use rayon::prelude::*;
use std::cmp;
use std::env;
use std::str::FromStr;

//Try this out, since rust is good at it. 
//predicate sum, overall MCSS, and the thing like predicate sum but on the end instead of the beginning.
//lets call it endicate sum
//you then join the calls by taking max of lefthalf MCSS, righthalf MCSS, and lefthalf's endicate + righthalfs predicate
//you also have to find the new predicate sum and endicate sum
fn MCSS(a:&[i32], left:u32, right:u32) -> (i32,i32,i32) {
	if right-left < 64{

	}
	let mid = (left + right)/2;
	let (leftpred, leftsum, leftright) = MCSS(a,left,mid);
	let (rightpred, rightsum, rightright) = MCSS(a,mid,right);
	//disregard this function
	(12,12,12)
}

//recursive multiply reduce for matrix mult
// let vector1 = //some vector
// let vector2 = //some other vector
// recursively_multiply_reduce(&vector1, &vector2)
fn recursively_multiply_reduce(a: &[i32], b:&[i32])-> i32 {
	if a.len() < 512 { //SOME base case
		return iter_multiply_reduce(a,b);
	}
	let (al,ar) = a.split_at(a.len()/2);
	let (bl,br) = b.split_at(b.len()/2);

	let left = recursively_multiply_reduce(al,bl);
	let right = recursively_multiply_reduce(ar,br);

	left + right
}

//using paralel for
fn iter_multiply_reduce(a: & [i32], b:& [i32])-> i32{
	a.iter()
		.zip(b.iter())
		.map(|(a,b)| a * b) //note the parentheis around (a,b)
		.fold(0,|a,b| a + b)
}

fn increment_all_par(input: &mut [i32]) { //generic test of parallel for
    input.par_iter_mut()
         .for_each(|p| *p += 1);
}

fn increment_all_seq(input: &mut [i32]){
	for element in input{
		*element = *element + 1;
	}
}

fn par_sum_iter(input: &mut [i32]) -> i32 { //this performs worse than recursive
    input.par_iter()
    		.map(|a| *a) //I can't believe this step is necessary
        	.sum()
}

fn par_rec_sum(input: &[i32])->i32{
	if input.len() < 1024{ //base case
		return input.iter().fold(0,|a,b| a+b); 
	}
	let (left,right) = input.split_at(input.len()/2);
	let (a, b) = rayon::join(
            || par_rec_sum(left),
            || par_rec_sum(right));
    a + b
}

fn seq_recursive_sum(input: &[i32])->i32{
	if input.len() < 1024{ //base case
		return input.iter().fold(0,|a,b| a+b); 
	}
	let (left,right) = input.split_at(input.len()/2);
	let a = seq_recursive_sum(left);
    let b = seq_recursive_sum(right);
    a + b
}

fn seq_sum_of_array1(input: &mut [i32]) -> i32 {
	input.iter()
		 .fold(0,|a,b| a+b) //because sum isn't stable
}
fn seq_sum_of_array2(input: &mut [i32]) -> i32 {
	let sum = input.iter().fold(0,|sum,x| sum + x); //slightly different semantics, is it the same as 1?
	sum
}
fn seq_sum_of_array3(input: &mut [i32]) -> i32 {
	let mut x:i32 = 0;
	for item in input{
		x = x + *item;
	}
	x
}
fn seq_sum_of_array4(input: &mut [i32]) -> i32 {
	let mut x:i32 = 0;
	for i in 0..input.len(){
		x = x + input[i];
	}
	x
}






fn main() {
	
	let mut kilos:usize = 100;
	let mut reps = 1; //might be edited so that we repeat reps times, average results.

	let mut args = env::args().skip(1);
	while let Some(arg) = args.next() {
        if arg == "--reps" {
            if let Some(reps_arg) = args.next() {
                match usize::from_str(&reps_arg) {
                    Ok(v) => reps = v,
                    Err(_) => {
                        println!("invalid argument `{}`, expected integer", reps_arg);
                    }
                }
            } else {
                println!("missing argument to --reps");
            }
        } else {
            match usize::from_str(&arg) {
                Ok(v) => kilos = v,
                Err(_) => {
                    println!("invalid argument `{}`, expected integer", arg);
                }
            }
        }
    }
	let truesize = 1024 * (kilos as i32);
	let mut start = time::precise_time_ns();
    //println!("Hello, world!");
    let mut time = time::precise_time_ns() - start;
    //println!("printing \"Hello, world!\" took {} ns", time);

    // Just unwrap as nobody else ought to be initializing the
    // thread-pool by this time.
    rayon::initialize(rayon::Configuration::new().set_num_threads(1)).unwrap();
    // All elements can be initialized to the same value
    let mut vector: Vec<i32> = (0..500).collect();
    start = time::precise_time_ns();
    increment_all_par(&mut vector);
    time = time::precise_time_ns() - start;
    //println!("incrementing 500 times in parallel took {} ns", time);


    //println!("now it's time for the real tests!");

    //println!("The vectors used are all of size {} kb", kilos);

    //in case I later decide to average out multiple iterations
    let mut seq_incr_time = 0;
    let mut par_incr_time = 0;
    let mut par_sum_rec_time = 0;
    let mut seq_sum_rec_time = 0;
    let mut seq_sum_iter_time = 0;
    let mut par_sum_iter_time = 0;

    let mut i = 0;
    while i < reps{
		vector = (0..truesize).collect();
		start = time::precise_time_ns();
		increment_all_seq(&mut vector);
		seq_incr_time += time::precise_time_ns() - start;

		let mut vector2: Vec<i32> = (0..truesize).collect();
		start = time::precise_time_ns();
		increment_all_par(&mut vector2);
		par_incr_time += time::precise_time_ns() - start;
		assert_eq!(vector,vector2);

		let mut lowvec: Vec<i32> = vec![1;truesize as usize]; //so that the "Add" doesnt overflow
		start = time::precise_time_ns();
		let sum1 = par_rec_sum(&mut lowvec);
		par_sum_rec_time += time::precise_time_ns() - start;

		let mut lowvec2: Vec<i32> = vec![1;truesize as usize]; //so that the "Add" doesnt overflow
		start = time::precise_time_ns();
		let sum2 = seq_recursive_sum(&mut lowvec2);
		seq_sum_rec_time += time::precise_time_ns() - start;
		assert_eq!(sum1,sum2);

		//test all sequential ways of iterating, choose the fastest
		start = time::precise_time_ns();
		let j = seq_sum_of_array1(&mut lowvec);
		let time1 = time::precise_time_ns() - start;
		start = time::precise_time_ns();
		let jj = seq_sum_of_array2(&mut lowvec);
		let time2 = time::precise_time_ns() - start;
		start = time::precise_time_ns();
		let jjj = seq_sum_of_array3(&mut lowvec);
		let time3 = time::precise_time_ns() - start;
		start = time::precise_time_ns();
		let jjjj = seq_sum_of_array4(&mut lowvec);
		let time4 = time::precise_time_ns() - start;
		seq_sum_iter_time += cmp::min(cmp::min(time1,time2),cmp::min(time3,time4));

		start = time::precise_time_ns();
		let jjjjj = par_sum_iter(&mut lowvec);
		par_sum_iter_time += time::precise_time_ns() - start;
		//ensure that the compiler didn't "optiimze away" anything.
		assert_eq!(j,jj); 
		assert_eq!(j,jjj);
		assert_eq!(j,jjjj);
		assert_eq!(j,jjjjj);
		i = i + 1;
	}
	par_incr_time = par_incr_time/(reps as u64);
	seq_incr_time = seq_incr_time/(reps as u64);
	par_sum_rec_time = par_sum_rec_time/(reps as u64);
	seq_sum_rec_time = seq_sum_rec_time/(reps as u64);
	par_sum_iter_time = par_sum_iter_time/(reps as u64);
	seq_sum_iter_time = seq_sum_iter_time/(reps as u64);

	//println!("Parallel time of increment   : {}", par_incr_time);
    //println!("Sequential time of increment : {}", seq_incr_time);
    //println!("speedup of increment      : {}",  ((seq_incr_time) as f64) / ((par_incr_time) as f64));
	//println!("Parallel time of recursive sum   : {}", par_sum_rec_time);
    //println!("Sequential time of recursive sum : {}", seq_sum_rec_time);
    //println!("speedup of recursive sum      : {}",  ((seq_sum_rec_time) as f64) / ((par_sum_rec_time) as f64));
	//println!("Parallel time of iterative sum   : {}", par_sum_iter_time);
    //println!("Sequential time of iterative sum : {}", seq_sum_iter_time);
    //println!("speedup of iterative sum      : {}",  ((seq_sum_iter_time) as f64) / ((par_sum_iter_time) as f64));
	println!("{},{},{},{},{},{}",par_incr_time, seq_incr_time, par_sum_rec_time, seq_sum_rec_time, par_sum_iter_time, seq_sum_iter_time);
}
