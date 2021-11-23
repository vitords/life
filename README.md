# Conway's Game of Life, but worse

You probably know it, but just in case: [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life)

Someone influenced me to try my hand at this, and so I decided to do it in Rust because I barely know Rust.

It kind of works, so there's that.

## If you dare to try it
```sh
$ cargo run
```

This should evolve as:
```
Start  Gen 0  Gen 1  Gen 2
 000    010    000    010
 111    010    111    010
 000    010    000    010
 ```

 ## Test
 ```
 $ cargo test
 ```