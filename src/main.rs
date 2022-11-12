mod Adaline;

fn main() {
    let and = [-1., 1., -1., -1.];
    let or = [-1., 1., 1., 1.];
    let xor = [-1., -1., 1., 1.];

    let mut adaline = Adaline::Adaline::new();

    adaline.cria_treinamento(and);
    adaline.treinamento(1000000);
    //adaline.apresenta_resultados();

    let mut x1: f64 = -1.5;
    while x1 < 1.5 {
        let mut x2: f64 = -1.5;
        while x2 < 1.5 {
            if (x1.abs() - 1.0).abs() < 0.2 && (x2.abs() - 1.0).abs() < 0.2 {
                println!("255 0 0 ");
            } else if adaline.propaga2(x1, x2) < 0. {
                println!("255 255 0 ");
            } else {
                println!("0 255 255");
            }
            x2 += 0.01;
        }
        x1 += 0.01;
    }
}
