fn main() {
	let v: Vec<i32> = vec![1,2,3];
	let a = 2;
	
	let result = increment_by(&v, a);
	
	println!("{}", a);
	println!("{:?}", &v);
	println!("{:?}", &result);
}

pub fn increment_by(v: &Vec<i32>, a: i32)-> Vec<i32>{
    let mut new_v: Vec<i32> = v.clone();
    for i in 0..new_v.len(){
        new_v[i] = new_v[i] + a;
    }
    new_v
}