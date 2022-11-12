use libm::tanh;

pub(crate) const GANHO: f64 = 100.0;

#[derive(Copy, Clone)]
pub struct Adaline {
    pub(crate) w: [f64; 2],
    pub(crate) x: [[f64; 2]; 4],
    pub(crate) t: [f64; 4],
    pub(crate) b: f64,
    pub(crate) y: f64,
    pub(crate) soma: f64,
    pub(crate) eta: f64,
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
            t: [0., 0., 0., 0.],
            b,
            y,
            soma,
            eta,
        }
    }

    pub(crate) fn treinamento(&mut self, maxiterates: i32) -> () {
        let mut y_interm: f64;

        for k in 1..maxiterates {
            let mut hits: i32 = 0;

            println!("It = {}", k);

            for i in 0..4 {
                y_interm = self.propaga(i);

                self.y = Adaline::f(y_interm);

                self.atualiza_pesos(i, y_interm);

                if self.y == self.t[i] {
                    hits += 1;
                } else {
                    self.atualiza_bias(i, y_interm);
                }
            }

            if hits == 4 {
                println!("Aprendizado concluido com {} iterações", k);
                break;
            }
        }
    }

    pub(crate) fn propaga(self, i: usize) -> f64 {
        let mut soma = 0.;

        for j in 0..2 {
            soma += self.x[i][j] * self.w[j];
        }

        soma + self.b
    }

    pub(crate) fn propaga2(self, x1: f64, x2: f64) -> f64 {
        let mut soma = 0.;

        soma = x1 * self.w[0] + x2 * self.w[1];

        soma + self.b
    }

    pub(crate) fn atualiza_pesos(&mut self, i: usize, y_res: f64) -> () {
        for j in 0..2 {
            self.w[j] += self.eta * (self.t[i] - y_res) * self.x[i][j];
        }
    }

    pub(crate) fn atualiza_bias(&mut self, i: usize, y_res: f64) -> () {
        //b+=(t[i]-y_res)*eta;
        self.b += (self.t[i] - y_res) * self.eta;
    }
    /*
    void Adaline::Apresenta_Resultados()
    {
        for (int l=0; l<4; l++)
            cout << l << "-th x[0]=" << x[l][0]<<" x[1]=" << x[l][1] <<" saida=" << f(Propaga(l))<< endl;
    }
    */

    pub(crate) fn apresenta_resultados(&mut self) -> () {
        for l in 0..4 {
            println!(
                "{} -th x[0]= {} ; x[1]= {} ; saida = {}",
                l,
                self.x[l][0],
                self.x[l][1],
                Adaline::f(self.propaga(l))
            );
        }
    }

    pub(crate) fn cria_treinamento(&mut self, a: [f64; 4]) -> () {
        for i in 0..4 {
            self.t[i] = a[i];
        }
    }

    pub(crate) fn f(arg: f64) -> f64 {
        tanh(arg * GANHO)
    }
}
