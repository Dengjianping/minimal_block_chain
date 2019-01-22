# minimal_block_chain
[![Build Status](https://travis-ci.com/Dengjianping/minimal_block_chain.svg?branch=master)](https://travis-ci.com/Dengjianping/minimal_block_chain)

## This is a block-chain toy inspired by this [post](https://hackernoon.com/learn-blockchains-by-building-one-117428612f46?gi=8e1bb887685f) titled [Learn Blockchains by Building One](https://hackernoon.com/learn-blockchains-by-building-one-117428612f46?gi=8e1bb887685f), which use python to implement a block-chain toy, just for learning. So I tried to use rust to do what python does.
I choose Actix-web as back-end, redis as database.



### How to build

#### Prepositions
1. Nightly rust.
2. Redis installed.
3. CUDA installed(at least 8.0, compute capability at least 3.5).
   - This is for accelerating mining, I implemented some encrypted algorithms like sha256/384/512.
4. Cmake(at least 3.8).
5. Gcc(5.0 would be better).

#### Build && Run
1. Configure the block_chain.toml file
- If you want cpu to mine the block, use following command the project

```
cargo run --release
```

- But if you want to try CUDA to mine the block, use this command to feel the feeling of flying

```
cargo run --features "cuda" --release
```