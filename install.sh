cd ..

mkdir community

cd community

# clone 
git clone git@github.com:xiyu1984/stark-verifier.git
git clone git@github.com:xiyu1984/halo2-solidity-verifier.git

cd ..

# clone fri-kzg-verifier
git clone git@github.com:Omniverse-Web3-Labs/fri-kzg-verifier.git

# clone zk-6358 and executor
git clone git@github.com:Omniverse-Web3-Labs/zk-omni-executor.git
cd zk-omni-executor
git checkout gas
git branch
cd ..
git clone git@github.com:Omniverse-Web3-Labs/zk-6358.git
cd zk-6358
git checkout gas
git branch
cd ..

# clone plonky2
git clone git@github.com:xiyu1984/plonky2.git
git clone git@github.com:xiyu1984/plonky2-ecdsa.git
git clone git@github.com:xiyu1984/plonky2-u32.git
git clone git@github.com:xiyu1984/plonky2-keccak256.git