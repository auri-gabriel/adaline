use libm::tanh;

const GANHO: f64 = 100.0;

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
    pub fn new() -> Self {
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

        let b = 0.;

        let y = 0.;

        let eta = 0.1;

        let soma = 0.0;

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

    fn treinamento(&mut self, maxiterates: i32) -> () {
        let mut y_interm: f64;

        for k in 0..maxiterates {
            let mut hits: i32 = 0;

            for i in 0..4 {
                y_interm = self.propaga(i);

                self.y = Adaline::f(y_interm);

                self.atualiza_pesos(i, y_interm);

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

    fn propaga(&mut self, i: usize) -> f64 {
        self.soma = 0.;

        for j in 0..2 {
            self.soma += self.x[i][j] * self.w[j];
        }

        self.soma + self.b
    }

    fn atualiza_pesos(&mut self, i: usize, y_res: f64) -> () {
        for j in 0..2 {
            self.w[j] += self.eta * (self.d[j] - y_res) * self.x[i][j];
        }
    }

    fn atualiza_bias(&mut self, i: usize) -> () {
        self.b += self.eta * (self.d[i] - 0.);
    }
    /*
    void Adaline::Apresenta_Resultados()
    {
        for (int l=0; l<4; l++)
            cout << l << "-th x[0]=" << x[l][0]<<" x[1]=" << x[l][1] <<" saida=" << f(Propaga(l))<< endl;
    }
    */

    fn apresenta_resultados(&mut self) -> () {
        for l in 0..4 {
            println!(
                "{} -th x[0]= {} ; x[1]= {} ; saida = {}",
                l,
                self.x[l][0],
                self.x[l][0],
                Adaline::f(self.propaga(l))
            );
        }
    }

    fn cria_treinamento(&mut self, a: [f64; 4]) -> () {
        for i in 0..4 {
            self.d[i] = a[i];
        }
    }

    fn f(arg: f64) -> f64 {
        tanh(arg * GANHO)
    }
}

fn main() {
    let and = [-1., 1., -1., -1.];
    let or = [-1., 1., 1., 1.];

    let mut adaline = Adaline::new();

    adaline.cria_treinamento(or);
    adaline.treinamento(1000000);
    adaline.apresenta_resultados();
}
