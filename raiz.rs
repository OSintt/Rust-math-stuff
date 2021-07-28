fn main() {
	let numero: i64 = 100;
	let mut vec = Vec::new();

	for i in 0..numero {
		if i * i == numero {
			println!("La raíz cuadrada de {} es {}", numero, i);
			vec.push(i);
			break;
		}		
	}
	if vec.len() == 0 {
		println!("{} no tiene raíz cuadrada", numero);
	}


}