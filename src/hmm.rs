/// 1. Implement functions that can read A and B and can generate the struct
/// 2. Implement functions that can read A and can generate some part of the struct
/// HMM with A (transition probability matrix N x N), B (emission probability matrix N x T)
/// N as number of hidden states, M is number of unique observations and T is length of observations
/// pie as initial probabilities


struct Hmm {
    A: Vec<Vec<f32>>,
    B: Vec<Vec<f32>>,
    pie: Vec<f32>,
    alpha: Vec<Vec<f32>>,
    beta: Vec<Vec<f32>>,
    N: u32,
    T: u32,
    M: u32
}

impl Hmm {

    fn read_A() {
        // read transition probabilities from some file
        todo!()
    }

    fn read_B() {
        // read emission probabilities from some file
        todo!()
    }


    fn initialize() {
        todo!()

    }


    fn forward(&mut self) {
        for i in 0..N {
            alpha[i][0] = pie[i] * beta[i] * 

        todo!()
    }



}


