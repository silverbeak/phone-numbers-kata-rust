# Just for fun...
I created a solution for [Emily Bache's Phone Numbers Kata](https://github.com/emilybache/Phone-Numbers-Kata), since it sounded like a nice project to solve as part of my Rust-learning process.
The files in the assets-folder is shamelessly stolen from her repository.

I encourage you to solve the problem on your own before looking at my solution. And perhaps you shouldn't look at my solution at all, since I'm just learning this language, and the code is probably anything but perfect.

However, I'm happy with the speed. I run the 65535 line file in just over 0.4 seconds on my (oldish) MBP. This is truly where Rust shines.

# Running the code
Make sure you have Cargo installed, then it's a matter of 
```sh
> cargo run ./src/assets/phone_data_65535.txt
```

To run the optimised version, and time it: 
```sh
> cargo build
> time ./target/release/phone-nr ./src/assets/phone_data_65535.txt
```