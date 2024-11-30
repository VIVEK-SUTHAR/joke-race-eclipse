
### First, Install Solana CLI and Rust

Then run
```
chmod 777 ./cleanup.sh
```


Now run
```
./cleanup.sh
```

This will bootstrap a new Solana test ledger to test and fund account with 1000 sol

then run
```
anchor test --provider.cluster http://127.0.0.1:8899 --provider.wallet ./a.json
```
