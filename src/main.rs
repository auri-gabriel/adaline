mod adaline;

fn main() -> std::io::Result<()> {
    let and = [-1., 1., -1., -1.];
    let or = [-1., 1., 1., 1.];
    let xor = [-1., -1., 1., 1.];

    let mut adaline = adaline::Adaline::new();

    adaline.cria_treinamento(and);
    adaline.treinamento(1000000);
    //adaline.apresenta_resultados();

    print!("P3\n301 301\n255\n");
    let mut x1: f64 = -1.5;
    for _ in 0..301 {
        let mut x2: f64 = -1.5;
        for _ in 0..301 {
            if (x1.abs() - 1.0).abs() < 0.2 && (x2.abs() - 1.0).abs() < 0.2 {
                print!("255 0 0 ");
            } else if adaline.propaga2(x1, x2) < 0. {
                print!("255 255 0 ");
            } else {
                print!("0 255 255 ");
            }

            x2 += 0.01;
        }
        x1 += 0.01;
    }

    Ok(())
}
