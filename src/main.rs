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
    fn treinamento(self, maxiterates: i32) -> () {
        let y_intern: f64;

        for k in 0..maxiterates {
            let hits: i32 = 0;

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

    fn propaga(self, i: usize) -> f64 {
        self.soma = 0.;

        for j in 0..2 {
            self.soma += self.x[i][j] * self.w[j];
        }

        self.soma + self.b
    }

    fn atualiza_pesos(self, i: usize, y_res: f64) -> () {
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
    println!("Hello, world!");
}
