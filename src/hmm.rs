/// 1. Implement functions that can read A and B and can generate the struct
/// 2. Implement functions that can read A and can generate some part of the struct
/// HMM with A (transition probability matrix N x N), B (emission probability matrix N x T)
/// N as number of hidden states, M is number of unique observations and T is length of observations
/// pie as initial probabilities
/// O is observation sequence


pub struct Hmm {
    pub a: Vec<Vec<f32>>, // N by N 
    pub b: Vec<Vec<f32>>, // N by M 
    pub o: Vec<usize>, // T 
    pub pie: Vec<f32>, //N 
    pub alpha: Vec<Vec<f32>>, // T by N  
    pub beta: Vec<Vec<f32>>, // N by M
    pub c: Vec<f32>, // T 
    pub n: usize,
    pub t: usize,
    pub m: usize 
}

impl Hmm {

    fn read_a() {
        // read transition probabilities from some file
        todo!()
    }

    fn read_b() {
        // read emission probabilities from some file
        todo!()
    }


    fn initialize() {
        todo!()

    }


    fn loglikelihood(&mut self) -> f32 {
        -1.0 * self.c.iter().map(|x| f32::log10(*x)).sum::<f32>()
    }


    fn backward(&mut self) {

    }


    pub fn forward(&mut self) {

        let mut sum = 0.0;
        for i in 0..self.n {
            self.alpha[0][i] = self.pie[i] * self.b[i][self.o[0]];
            sum += self.alpha[0][i];
        }

        self.c.push(1.0 / sum);

        for i in 0..self.n {
            self.alpha[i][0] *=  self.c[0];
        }

        for t in 1..self.t {
            let mut sum = 0.0;
            for i in 0..self.n {
                let mut val = 0.0;
                for j in 0..self.n {
                    val += self.alpha[j][t-1] * self.a[j][i];
                }
                self.alpha[i][t] = val * self.b[i][self.o[t]];
                sum += self.alpha[i][t];
            }

            self.c.push(1.0 / sum);
            for i in 0..self.n {
                self.alpha[i][t] *= self.c[t];
            }

        }
            
    }


    fn train(&mut self) {

    }

}


