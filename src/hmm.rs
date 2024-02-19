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
    pub beta: Vec<Vec<f32>>, // T by N  
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




    pub fn loglikelihood(&self) -> f32 {
        -1.0 * self.c.iter().map(|x| f32::log10(*x)).sum::<f32>()
    }


    pub fn backward(&mut self) {

        self.beta = vec![vec![0.0; self.n]; self.t];
        let mut beta_row = vec![];
        for _i in 0..self.n {
            beta_row.push(self.c[self.t - 1]);
        }

        self.beta[self.t - 1] = beta_row;

        for t in (0..self.t-1).rev() {
            let mut beta_row: Vec<f32> = vec![];
            for i in 0..self.n {
                let mut val:f32 = 0.0;
                for j in 0..self.n {
                    val += self.a[i][j] * self.b[j][self.o[t+1]] * self.beta[t + 1][j]
                }
                val *= self.c[t];
                beta_row.push(val);
            }
            self.beta[t] = beta_row;
        }

    }


    pub fn forward(&mut self) {

        let mut sum = 0.0;
        let mut alpha_row = vec![];
        for i in 0..self.n {
            alpha_row.push(self.pie[i] * self.b[i][self.o[0]]);
            sum += alpha_row[i];
        }

        self.c.push(1.0 / sum);

        let alpha_row: Vec<f32> = alpha_row.iter_mut().map(|x| *x * self.c[0]).collect();
        
        self.alpha.push(alpha_row);

        for t in 1..self.t {
            let mut sum = 0.0;
            let mut alpha_row: Vec<f32> = vec![];
            for i in 0..self.n {
                let mut val = 0.0;
                for j in 0..self.n {
                    val += self.alpha[t-1][j] * self.a[j][i];
                }
                alpha_row.push(val * self.b[i][self.o[t]]);
                sum += alpha_row[i];
            }

            self.c.push(1.0 / sum);
            let alpha_row: Vec<f32> = alpha_row.iter_mut().map(|x| *x * self.c[t]).collect();
            self.alpha.push(alpha_row);
        }
            
    }


    fn estimate(&mut self) {
        // Try implementating this in one attempt
        // xi and gamma
        let mut xi: Vec<Vec<f32>> = vec![];
        for i in 0..self.n {
            let mut row: Vec<f32> = vec![];
            for j in 0..self.n {
                let mut val:f32 = 0.0;
                for t in 0..self.t-1 {
                    val += self.alpha[t][i] * self.a[i][j] * self.b[j][self.o[t+1]] * self.beta[t+1][j]
                }
                row.push(val);
            }
            xi.push(row);
        }
        println!("matrix xi = {:?}", &xi);

        let mut gamma: Vec<Vec<f32>> = vec![];
        for t in 0..self.t-1 {
            let mut row: Vec<f32> = vec![];
            for i in 0..self.n {
                let mut val:f32 = 0.0;
                for j in 0..self.n {
                    val += self.alpha[t][i] * self.a[i][j] * self.b[j][self.o[t+1]] * self.beta[t+1][j];
                }
                row.push(val);
            }
            gamma.push(row);
        }
        gamma.push(self.alpha[self.t - 1].to_owned());
        println!("matrix gamma = {:?}", &gamma);
        self.pie = gamma[0].to_owned();

        let mut gamma_t_1: Vec<f32> = vec![];
        for i in 0..self.n {
            let mut val: f32 = 0.0;
            for j in 0..self.t-1 {
                val += gamma[j][i];
            }
            gamma_t_1.push(val);
        }

               // re-estimate a[i][j]
        for i in 0..self.n {
            for j in 0..self.n {
                self.a[i][j] = xi[i][j] / gamma_t_1[i];
            }
        }

        println!("matrix b = {:?}", self.b);
        // re-estimate b[j][k]
        for i in 0..self.n {
            let mut denom: f32 = 0.0;
            for t in 0..self.t {
                denom += gamma[t][i];
            }

            for j in 0..self.m {
                let mut numer: f32 = 0.0;
                for t in 0..self.t {
                    if self.o[t] == j {
                        numer += gamma[t][i];
                    }
                }
                self.b[i][j] = numer / denom;
            }
        }

        println!("matrix b = {:?}", self.b);
    }

    pub fn train(&mut self) {
        self.estimate();

    }

}


