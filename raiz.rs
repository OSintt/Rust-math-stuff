fn main() {
	let numero: i32 = 100;
	let mut vec = Vec::new();

	for i in 0..numero * 100 {
		let i = i as f32 * 0.01;
		println!("{:?}", i);
		if i * i == numero as f32 {
			println!("La raíz cuadrada de {} es {}", numero, i);
			vec.push(i);
			break;
		}		
	}
	if vec.len() == 0 {
		println!("{} no tiene raíz cuadrada", numero);
	}


}