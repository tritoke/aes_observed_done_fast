# AES Obsvered - Done Fast

This challenge gave you an endpoint which returned `plaintext`, `voltage` pairs for a given AES Encryption with a static key and asks you to recover the key to get the flag.

## Methodology

This attack is called a Correlation Power Analysis attack:

1. Acquire some samples from the endpoint, I got 2.4 million but in the end only needed 800...
2. For each byte n of the key:
    1. For each possible candidate byte for byte n of the key (0-255):
        1. Xor the nth plaintext byte with the key guess for all plaintexts
        2. Apply the SBOX to these bytes
        3. Calculate the hamming weight of these bytes
        4. Calculate the Pearson Correlation Coefficient between these numbers and the measured voltages
    2. output the candidate byte with the strongest correlation with the measured voltages
3. Print the key


## Benchmarking

Python implementation took roughly 4 seconds:
```
2023/p4_finals/aes_observed
➜ time ./solve.py
100%|█████████████████████████████████████████████████████████| 32/32 [00:03<00:00,  8.38it/s]
b'p4{Oscilloscopes? Still_matter!}'
./solve.py  3.89s user 0.03s system 99% cpu 3.921 total
```

The Rust implementation runs instantly:
```
aes_observed_done_fast on  main [?] is 📦 v0.1.0 via 🦀 v1.72.0
➜ hyperfine -N target/release/aes_observed_done_fast
Benchmark 1: target/release/aes_observed_done_fast
  Time (mean ± σ):       5.1 ms ±   0.5 ms    [User: 45.3 ms, System: 16.5 ms]
  Range (min … max):     4.4 ms …   7.7 ms    412 runs
```
