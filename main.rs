struct Triangulo {
	cateto1: f64,
	cateto2: f64
}

impl Triangulo {
	fn get_hipotenusa(&self) -> f64 {
		let catetos_multi = self.cateto1 + self.cateto2;
		catetos_multi.sqrt()
	}
}

fn main() {
	let new_tr = Triangulo {
		cateto1: 20.0,
		cateto2: 30.0,
	};

	println!("La hiportenusa de este triángulo es: {:?}", new_tr.get_hipotenusa());
}