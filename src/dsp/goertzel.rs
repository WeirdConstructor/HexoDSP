
#[derive(Debug, Copy, Clone, Default)]
pub struct Goertzel{
    target_freq: f32
    reference_tuning: f32 // assumed that target freqs will be int multiples of ref tuning
    ideal_buffsize: u16 // calculated with respect to target freq and ref tuning
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
        s.ideal_buffsize = floor(2 * 44100 / reftune) as u16
        s.buff = VecDeque::with_capacity(s.ideal_buffsize)
        s
    }

    #[inline]
    pub fn tick(&mut self, input: f32) -> f32 {
        let x0 = input;

        // return 0 until the buffer is full
        if(this.buff.len() < this.ideal_buffsize){
            this.buff.push_back(x0);
            return 0;
        }
        this.buff.pop_front();
        this.buff.push_back(x0);
        
        let k = floor(0.5 * ((this.ideal_buffsize * target_freq)/44100)) as f32
        let w = (2 * pi * k /this.ideal_buffsize) as f32
        let c = f32::cos(w)
        let coeff = 2 * c

        let Q0 = 0.0
        let Q1 = 0.0
        let Q2 = 0.0
        
        for i in 0..this.ideal_buffsize{
            Q0 = coeff* Q1 - Q2 + this.buff.get(i)
            Q2 = Q1
            Q1 = Q0
        }
        mag_squared = (Q1.pow(2) + (Q2.pow(2)) - (Q1*Q2*coeff)) as f32

        f32::sqrt(mag_squared)
    }
}