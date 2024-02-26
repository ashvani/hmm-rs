use std::vec;

mod hmm;

fn main() {

    let a = vec![vec![0.7, 0.3], vec![0.4, 0.6]];
    let b = vec![vec![0.1, 0.4, 0.5], vec![0.7, 0.2, 0.1]];
    let pie = vec![0.6, 0.4];
    let o = vec![0, 1, 0, 2];
    let mut model = hmm::Hmm::new(2, 3, o)
        .emission_prob(b)
        .transition_prob(a)
        .initial_prob(pie);

    let value: Vec<f32> = vec![];
    println!("{:?}", value);
    println!("{:?}", value.is_empty());
    model.forward();
    println!("loglikelihood = {}", model.loglikelihood());

    model.train();
}

