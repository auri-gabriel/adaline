use libm::tanh;

pub(crate) const GAIN: f64 = 100.0;

#[derive(Copy, Clone)]
pub struct Adaline {
    pub(crate) w: [f64; 2],
    pub(crate) x: [[f64; 2]; 4],
    pub(crate) t: [f64; 4],
    pub(crate) b: f64,
    pub(crate) y: f64,
    pub(crate) sum: f64,
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

        let sum = 0.0;

        Self {
            w,
            x,
            t: [0., 0., 0., 0.],
            b,
            y,
            sum,
            eta,
        }
    }

    pub(crate) fn training(&mut self, max_iterations: i32) -> () {
        let mut intermediate_output: f64;

        for k in 1..max_iterations {
            let mut hits: i32 = 0;

            //println!("It = {}", k);

            for i in 0..4 {
                intermediate_output = self.propagate(i);

                self.y = Adaline::f(intermediate_output);

                self.update_weights(i, intermediate_output);

                if self.y == self.t[i] {
                    hits += 1;
                } else {
                    self.update_bias(i, intermediate_output);
                }
            }

            if hits == 4 {
                //println!("Learning completed with {} iterations", k);
                break;
            }
        }
    }

    pub(crate) fn propagate(self, i: usize) -> f64 {
        let mut sum = 0.;

        for j in 0..2 {
            sum += self.x[i][j] * self.w[j];
        }

        sum + self.b
    }

    pub(crate) fn propagate2(self, x1: f64, x2: f64) -> f64 {
        let sum: f64;

        sum = x1 * self.w[0] + x2 * self.w[1];

        sum + self.b
    }

    pub(crate) fn update_weights(&mut self, i: usize, y_res: f64) -> () {
        for j in 0..2 {
            self.w[j] += self.eta * (self.t[i] - y_res) * self.x[i][j];
        }
    }

    pub(crate) fn update_bias(&mut self, i: usize, expected_output: f64) -> () {
        self.b += (self.t[i] - expected_output) * self.eta;
    }

    pub(crate) fn show_results(&mut self) -> () {
        for l in 0..4 {
            println!(
                "{} -th x[0]= {} ; x[1]= {} ; output = {}",
                l,
                self.x[l][0],
                self.x[l][1],
                Adaline::f(self.propagate(l))
            );
        }
    }

    pub(crate) fn create_training(&mut self, a: [f64; 4]) -> () {
        for i in 0..4 {
            self.t[i] = a[i];
        }
    }

    pub(crate) fn f(arg: f64) -> f64 {
        tanh(arg * GAIN)
    }
}
