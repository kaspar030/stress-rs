use rustfft::{num_complex::Complex, FftPlanner};

fn main() {
    // Perform a forward FFT of size 1024

    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(1024);

    loop {
        let mut buffer = vec![
            Complex {
                re: 0.0f32,
                im: 0.0f32
            };
            1024
        ];
        fft.process(&mut buffer);
    }
}
