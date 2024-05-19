# ZK-6358-Runner

## Note

This project depends on the lib `zk-6358` and `zk-omni-executor`, both of which are not been open-sourced. 

## Prepare

### Rust 

```sh
rustup override set nightly
```

### Environment

- `apt install pkg-config`
- `apt-get install libssl-dev`
- `apt-get install libclang-dev`
- install `solc`
    - `sudo add-apt-repository ppa:ethereum/ethereum`
    - `sudo apt-get update`
    - `sudo apt-get install solc`


- Update `ahash` if using the latest `rustup` version (above `1.76.*`)
    
    ```sh

    cargo update ahash@0.8
    cargo update ahash@0.7

    ```

### Exec

- run in backend

    ```sh

    ./target/release/zk-6358-runner > ./zk-running.log 2>&1 &

    # check id
    jobs -l
    ps -ef|grep zk-6358-runner

    # check log
    tail -n 100 ./zk-running.log

    ```

### Env

- config swap
    - `/etc/sysctl.config`
        - `vm.swappiness = 30`
        - `sysctl -p`
    - swapon
        - `fallocate -l 32G /mnt/swap`
        - `chmod 0600 /mnt/swap`
        - `mkswap /mnt/swap`
        - `swapon /mnt/swap`
    - reset
        - `swapoff -a`
