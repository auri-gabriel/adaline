#[derive(Copy, Clone)]
pub struct Adaline {
    w: [f64; 2],
    x: [[f64; 2]; 4],
    d: [f64; 4],
    b: f64,
    y: f64,
    soma: f64,
    eta: f64,
}

impl Adaline {
    pub fn new(w: [f64; 2], x: [[f64; 2]; 4], b: f64, y: f64, soma: f64, eta: f64) -> Self {
        Self {
            w,
            x,
            d: [0., 0., 0., 0.],
            b,
            y,
            soma,
            eta,
        }
    }

    fn treinamento(mut self, maxiterates: i32) -> () {
        let mut y_intern: f64;

        for k in 0..maxiterates {
            let mut hits: i32 = 0;

            for i in 0..4 {
                y_intern = self.propaga(i);

                self.y = self.f(y_intern);

                self.atualiza_pesos(i, y_intern);

                if self.y >= self.d[i] {
                    hits += 1;
                } else {
                    self.atualiza_bias(i);
                }
            }

            if hits == 4 {
                println!("Aprendizado concluido com {} iterações", k)
            }
        }
    }

    fn propaga(mut self, i: usize) -> f64 {
        self.soma = 0.;

        for j in 0..2 {
            self.soma += self.x[i][j] * self.w[j];
        }

        self.soma + self.b
    }

    fn atualiza_pesos(mut self, i: usize, y_res: f64) -> () {
        for j in 0..2 {
            self.w[j] += self.eta * (self.d[j] - y_res) * self.x[i][j];
        }
    }

    fn atualiza_bias(self, i: usize) -> () {
        todo!()
    }
    fn apresenta_resultados(self) -> () {
        todo!()
    }

    fn cria_treinamento(mut self, a: [f64; 4]) -> () {
        for i in 0..4 {
            self.d[i] = a[i];
        }
    }

    fn f(&self, y_intern: f64) -> f64 {
        todo!()
    }
}

fn main() {
    let w = [0.0, 0.0];

    let mut x = [[0f64; 2]; 4];

    x[0][0] = -1.0;
    x[0][1] = -1.0;
    x[1][0] = 1.0;
    x[1][1] = 1.0;
    x[2][0] = -1.0;
    x[2][1] = 1.0;
    x[3][0] = 1.0;
    x[3][1] = -1.0;

    let d = [1., 1., 1., 1.];

    let b = 0.;

    let y = 0.;

    let eta = 0.1;

    let soma = 0.0;

    let adaline = Adaline::new(w, x, b, y, soma, eta);

    println!("Hello, world!");
}
