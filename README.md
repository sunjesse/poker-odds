# NLH Poker Equity Calculator
An equity calculator for NLH poker players figuring out how much to bet / call that is plus EV.

![](https://github.com/sunjesse/poker-odds/blob/main/demo/demo.gif)


**What makes it so fast?**
- **Multi-threaded goodness:** I used a branching algorithm with memoization synchronized across threads optimized for concurrent reads using DashMap / rwlocks - as the algorithm is read-heavy.
- **Lots of bit manipulation:** we use the first 52 bits in a u64 int to represent the state of the board. To derive the rank of each hand, it just involves bit manipulations which is fast.
- **Keep as much on the stack as possible:** related to above, by encoding boards and cards as u64 values, most of the data stays on the stack avoiding runtime heap allocations and reducing memory overhead.
- **SIMD acceleration:** Many of the bitwise computations are vectorizable, allowing us to use SIMD instructions to evaluate multiple bits in parallel. We use SIMD whenever we run the bit manipulation logic to derive a hand's rank.

As a standard benchmark throughout the development of this project, I considered generating the equity of the following state: 2 players, empty board -> 48 cards to select 5 from. The branching algorithm exhaustively checks every possible combination of 5 cards on the board and computes which player won. It then aggregates all these results into a final probability.

Note that 48 choose 5 is approximately ~1.7m -> the algorithm goes through a 5 layer search tree with ~1.7m leaf nodes.

I first built a proof-of-concept in Python - this setup above resulted in a runtime of approximately 60-70 seconds. After migrating to Rust, an unoptimized implementation still nets around the same time as the Python implementation.

After all these optimizations listed above, we were able to bring down the runtime to approximately 400ms. **Up to a 175x speed up!**

Try it yourself by going into `poker-odds-rs/crates/poker-odds-gui` and run `cargo run`. You'll need the `nightly` build as we are using SIMD instrinsics.
