# Solana_AgriChat_SmartContract
Solana Smart Contract to run AgriChat

## Pre Dependencies for each system

### Ubuntu:

- Step 1: Install ubuntu, goto terminal and type su 
- Step 2: Type visudo
- Step 3: Type <username>   ALL=(ALL:ALL)ALL
- Step 4: Install curl using command sudo apt install curl, check if the        terminal asks for adding home
- Step 5: Install VS code from Ubuntu Software
## Pre Dependencies must for all system( Check terminal window for environment config)

- Step 1: Install rust using https://www.rust-lang.org/tools/install, use sudo apt install rustc
- Step 2: Install node using https://nodejs.org/en/download/package-manager
- Step 3: Install Yarn using https://classic.yarnpkg.com/lang/en/docs/install/#debian-stable
- Step 4: Install solana cli using https://docs.solanalabs.com/cli/install
- Step 5: Install cargo : `sudo apt install cargo`
- Step 6: Install solana key pair to run program `solana-keygen new`
- Step 7: Copy the walley address and goto https://faucet.solana.com/ , paste the wallet address, and generate atleast 2.5 SOL
- Step 8: Install Anchor using https://www.anchor-lang.com/docs/installation

### Dependencies
- check if package.json are pre installed and are in correct version.

### Deploy program

- `anchor build`
- `solana address -k target/deploy/solana_chat-keypair.json`
- Replace Program ID in `lib.rs` and `Anchor.toml`
- `anchor build` (again)
- `anchor deploy`

### Output:
![image](https://github.com/Tejesh1606/Solana_AgriChat_SmartContract/assets/96534599/6cb91d0a-5488-496e-8ba9-ea2779f813cb)

### Note:
- The smart contract does not have update and delete function for immutability.

### Post Step:
Copy the Program Id and paste it in this code https://github.com/Tejesh1606/Solana_AgriChat_FrontEnd , and run the code using its readme file.
