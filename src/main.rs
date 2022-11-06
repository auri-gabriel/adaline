pub struct Adaline {
    w: [f64; 2],
    x: [[f64; 4]; 2],
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

    fn propaga(self, i: i32) -> f64 {
        self.soma = 0.;

        for j in 0..2 {
            self.soma += self.x[i][j] * self.w[j];
        }

        self.soma + self.b
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
