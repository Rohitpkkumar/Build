# DLinktree

## Who are we ?
Team members :
Rohit Kuamr
Inderjeet Singh

## Introduction
DLinktree is a decentralized and immutable identity management system built on the Stellar blockchain. It aggregates a user's on-chain activities, such as degen activities, NFTs, Wordcel blogs, work resume, and other credentials, into a comprehensive profile.

## Features
- Create and update user profiles.
- Add and retrieve on-chain credentials.
- Ensure profile originality and prevent fake accounts.

## Project Details:
Building social consumer apps is challenging but essential for onboarding the next wave of crypto users. Identity in crypto is underutilized despite its potential to revolutionize ownership and reputation in web3.
DLinktree provides a one-page, comprehensive profile aggregating a user's on-chain activities. This includes degen activities, NFTs, Wordcel blogs, work resume, and other credentials. By leveraging Stellar's blockchain, we ensure secure, composable, and user-owned identity management, setting the foundation for a robust reputation system in the decentralized web.

## Vision
Our vision is to revolutionize online identity with Stellar Identity Hub, a decentralized and immutable "Linktree for web3." By deploying profiles on the Stellar blockchain, we ensure the originality and authenticity of each profile, preventing fake accounts and duplicate names. This innovation will create a trustworthy digital identity system, empowering users with true ownership and control over their online presence. Ultimately, Stellar Identity Hub will drive the next wave of crypto adoption, fostering a secure, transparent, and reputable web3 ecosystem.

## Project Structure

This repository uses the recommended structure for a Soroban project:
```text
.
├── contracts
│   └── hello_world
│       ├── src
│       │   ├── lib.rs
│       │   └── test.rs
│       └── Cargo.toml
├── Cargo.toml
└── README.md
```

- New Soroban contracts can be put in `contracts`, each in their own directory. There is already a `hello_world` contract in there to get you started.
- If you initialized this project with any other example contracts via `--with-example`, those contracts will be in the `contracts` directory as well.
- Contracts should have their own `Cargo.toml` files that rely on the top-level `Cargo.toml` workspace for their dependencies.
- Frontend libraries can be added to the top-level directory as well. If you initialized this project with a frontend template via `--frontend-template` you will have those files already included.


## Installation

### Prerequisites
- Node.js
- npm (Node Package Manager)
- Soroban CLI
- Rust
- A web browser (for the front-end)

### Step-by-Step Installation

1. **Clone the Repository**
    ```bash
    git clone https://github.com/Rohitpkkumar/Build.git
    cd Build
    ```

2. **Install Dependencies**
    ```bash
    npm install
    ```


3. **Deploy Smart Contracts**
    ```
    soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/hello_world.wasm \
  --source alice \
  --network testnet
    ```

4. **Set Up Front-End**
    - Navigate to the `frontend` directory:
      ```bash
      cd frontend
      ```


5. **Run the Application**
    - Install LiveServer Extenson on VS code and then click on Go Live


## Usage
- **Create Profile:** Fill out the profile creation form and submit.
- **Update Profile:** Modify existing profile details and save changes.
- **Add Credentials:** Add new on-chain credentials through the credentials form.
- **View Profile:** Browse profiles to view aggregated on-chain activities.

## Contributing
Please read `CONTRIBUTING.md` for details on our code of conduct and the process for submitting pull requests.

## License
This project is licensed under the MIT License - see the `LICENSE` file for details.

## Acknowledgements
- [Stellar](https://www.stellar.org) for their blockchain platform.