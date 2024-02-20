mod hmm;

fn main() {

    let a = vec![vec![0.7, 0.3], vec![0.4, 0.6]];
    let b = vec![vec![0.1, 0.4, 0.5], vec![0.7, 0.2, 0.1]];
    let pie = vec![0.6, 0.4];
    let alpha = vec![];
    let beta = vec![];
    let o = vec![0, 1, 0, 2];
    let mut model = hmm::HmmBuilder::default()
        .observations(o)
        .transition_prob(a)
        .emission_prob(b)
        .n(2)
        .m(3)
        .t(4)
        .c(vec![])
        .beta(beta)
        .alpha(alpha)
        .pie(pie)
        .build()
        .unwrap();

//    let mut model = hmm::Hmm{a,
//    b, alpha, beta, o, c: vec![],
//    t: 4, n: 2, m: 3, pie 
//    };
//
//    model.forward();
//    println!("log likelihood = {}", model.loglikelihood());
//    println!("alpha = {:?}", model.alpha);
//    model.backward();
//    println!("beta = {:?}", model.beta);
    model.train();
//    model.forward();
//    println!("log likelihood = {}", model.loglikelihood());
}

