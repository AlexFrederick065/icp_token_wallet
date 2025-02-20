# Setting Up and Using the ICP Token Wallet Backend

Follow these steps to clone the project, start the local DFX network, and interact with the ICP Token Wallet Backend.

## Prerequisites
Ensure you have the following installed:
- [DFX SDK](https://internetcomputer.org/docs/current/developer-docs/setup/install/)
- Rust and Cargo
- Git

---

## Steps to Setup and Run the Project

### 1. Clone the Repository
Open a Ubuntu terminal and run:

git clone <repository-url>
cd <project-directory>

### 2. Update Dependencies
Update Rust and Cargo dependencies:

cargo update

### 3. Start the Local DFX Network
Start the Internet Computer development network in the background:

dfx start --background

### 4. Deploy the Canisters
Deploy the project to the local DFX network:

dfx deploy

### 5. Deploy the Canisters
- Check Wallet Balance

dfx canister call icp_token_wallet_backend get_balance

- Receive Tokens

dfx canister call icp_token_wallet_backend receive_tokens '( "0x123456789abcdef", 100 )'

- Send Tokens

dfx canister call icp_token_wallet_backend send_tokens '( "0x223456789abcdef", 50 )'


