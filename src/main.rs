pub struct Adaline {
    w: [f64; 2],
    x: f64,
    d: [f64; 4],
    b: f64,
    y: f64,
    soma: f64,
    eta: f64,
}

impl Adaline {
    fn treinamento(maxiterates: i32) -> () {
        todo!()
    }

    fn propaga(i: i32) -> f64 {
        todo!()
    }

    fn atualiza_pesos(i: i32, y_im: f64) -> () {}

    fn atualiza_bias(i: i32) -> () {}

    fn apresenta_resultados() -> () {}

    fn cria_treinamento(self, A: [f64]) -> () {
        for i in 0..4 {
            self.d[i] = A[i];
        }
    }
}

fn main() {
    println!("Hello, world!");
}
