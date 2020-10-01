
fn fib(n: &mut i32)->i32{

	let mut vec: Vec<i32> = Vec::new();
	vec.push(1);
	vec.push(1);

	for i in 2..(*n+1){

		vec.push(vec[(i-1) as usize]+vec[(i-2) as usize]);

		println!("{}", i);
	}

	return vec[*n as usize];
}

fn main(){    

	let mut n = 20;

	println!("Fibronacci of {} = {}", n.clone(),fib(&mut n));

}