use std::collections::VecDeque;

const PI:f32 = std::f32::consts::PI;

#[derive(Debug, Clone, Default)]
pub struct Goertzel{
    pub target_freq: f32,
    reference_tuning: f32, // assumed that target freqs will be int multiples of ref tuning
    ideal_buffsize: usize, // calculated with respect to target freq and ref tuning
    buff: VecDeque<f32>
}

// Calculates an individual term of the Discrete Fourier Series
// implementation with notes taken from https://www.embedded.com/the-goertzel-algorithm/
impl Goertzel{
    pub fn new() -> Self {
        Default::default()
    }

    #[inline]
    pub fn new_with(tfreq:f32, reftune:f32) -> Self{
        let mut s = Self::new();
        s.target_freq = tfreq;
        s.reference_tuning = reftune;
        //you want the target frequencies to be integer multiples of sample_rate/N
        s.ideal_buffsize = (2.0 * 44100.0 / reftune).floor() as usize;
        s.buff = VecDeque::with_capacity(s.ideal_buffsize);
        s
    }

    pub fn reset(&mut self){
        self.buff.clear();
    }

    #[inline]
    pub fn tick(&mut self, input: f32) -> f32 {
        let x0 = input;

        // return 0 until the buffer is full
        if self.buff.len() < self.ideal_buffsize{
            self.buff.push_back(x0);
            return 0.0;
        }
        self.buff.pop_front();
        self.buff.push_back(x0);
        
        let k = (0.5 * ((self.ideal_buffsize as f32 * self.target_freq)/44100.0)).floor() as f32;
        let w = (2.0 * PI * k / self.ideal_buffsize as f32) as f32;
        let c = f32::cos(w);
        let coeff = 2.0 * c;

        let mut Q0;
        let mut Q1 = 0.0;
        let mut Q2 = 0.0;
        
        for i in 0..self.ideal_buffsize{
            Q0 = coeff* Q1 - Q2 + self.buff.get(i).unwrap();
            Q2 = Q1;
            Q1 = Q0;
        }
        let mag_squared = (Q1.powf(2.0) + (Q2.powf(2.0)) - (Q1*Q2*coeff)) as f32;

        f32::sqrt(mag_squared)
    }
}