# ZK-Omni-Executor

## Note

This project is under development and it depends on the lib `zk-6358`, which has not beed open-sourced. 

## Prepare

### Rust 

```sh
rustup override set nightly
```

### Environment

- `apt install pkg-config`
- `apt-get install libssl-dev`
- `apt-get install libclang-dev`


- Update `ahash` if using the latest `rustup` version (above `1.76.*`)
    
    ```sh

    cargo update ahash@0.8
    cargo update ahash@0.7

    ```

### Exec
