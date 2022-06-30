use std::time::{Instant};
use rand::Rng;

fn pss_5gnr_gen(n_id2: i16, data: &mut [i16]) {

    let mut x: [i16; 128] = [0; 128];
    let mut y: [i16; 128] = [0; 128];

    x[0] = 0;
    x[1] = 1;
    x[2] = 1;
    x[3] = 0;
    x[4] = 1;
    x[5] = 1;
    x[6] = 1;

    for n in 0..data.len() {
         if n <= 6 {
            y[n] = x[n]
        } else {
            y[n] = (y[n - 7 + 4] + y[n - 7]) % 2;
        }
    }
    
    for m in 0..data.len() {
        data[m] = 1 - 2 * y[(m + 43 * n_id2 as usize) % 127];

        if data[m] == 1 {
            data[m as usize] = 0;
        } else if data[m] == -1 {
            data[m as usize] = 1;
        }
    }

    // Another way of implementation
    /*
    for i in 0..data.len()  {
        x[i + 7] = (x[i + 4] + x[i]) % 2;
    }

    for i in 0..data.len() {
        y[i] = x[(i + 43 * n_id2 as usize) % 127];
    }

    for i in 0..data.len() {

        data[i] = 1 - 2 * y[i];

        if data[i] == 1 {
            data[i as usize] = 0;
        } else if data[i] == -1 {
            data[i as usize] = 1;
        }
    }
    */
}

fn pss_5gnr_conv(din: &mut [i16], dout: &mut [i16]) {

    let mut ct_words = 0;

    for ct_bits in 0..din.len() {
        if ct_bits == 16 * ct_words {
            ct_words = ct_words + 1;
        } else {
            dout[ct_words - 1] |= din[ct_bits] << (ct_bits - 16 * (ct_words - 1)); 
        }
    }

}


fn main() {

    let mut rng = rand::thread_rng();
    
    let mut pss_data: [i16; 128] = [0; 128];
    let n_id2_rng = rng.gen_range(0..3);

    // Benchmarking with random value of N_ID2 (0..3)
    let start = Instant::now();
    pss_5gnr_gen(n_id2_rng, &mut pss_data);
    let duration = start.elapsed();
    println!("Time elapsed in pss_nr_pregen() with n_id2 = {} is: {:?} ", n_id2_rng, duration);

    // Pack 1-bit into 16-bit values
    let mut pss_data_conv: [i16; 8] = [0; 8];
    pss_5gnr_conv(&mut pss_data, &mut pss_data_conv);

    // Display packed values from random N_ID2
    for i in 0..pss_data_conv.len() {
        println!("Packed in 16-bit words values: 0x{:X} ", pss_data_conv[i]);
    }


}
