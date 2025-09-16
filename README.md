# Lance: Decentralized Freelance Marketplace

## Problem
Centralized freelance platforms charge high fees (5 to 20%), with cross-border payments somethimes  delayed for several days and with an average cost of 6.04%. While also rely on dispute processes centralized in the same platform.

## Solution
**Lance** is a decentralized marketplace using Stellar's blockchain for secure, low-cost, instant global payments and smart contract-based escrow to protect users. Clients post jobs, fund escrow with USDC/XLM, and freelancers get paid automatically upon work approval or dispute resolutions, offering really lower fees, almost zero costs and times for transactions and a decentralized voting system based on game's theory and the Neural Quorum Governance system [link https://github.com/BlockScience/neural-quorum-governance].
- **Low Fees**: Minimal fees since we will focus our revenue model on yield through Blend Capital.
- **Fast Payments**: Freelancers can withdraw instantly to their wallets, avoiding conversion or cross-border bank delays.
- **Free Payments**: Transfers through Stellar have almost no cost even for cross-border payments.
- **Trustless Escrow**: Soroban smart contracts lock funds, releasing only on verified conditions, reducing dispute bias.
- **Transparency**: Our escrow and voting system is open-source and immutable while all transactions are public on Stellar Explorer.

This MVP demonstrates job/services posting, escrow funding, work submission, and payment release.

## Architecture
- **Frontend**: React (Next.js) for a simple UI where clients post jobs (title, budget in USDC/XLM) and freelancers accept/submit work. Hosted on Vercel for demo.
- **Backend (Light)**: Node.js with Stellar SDK for wallet interactions and job metadata storage (off-chain for simplicity, IPFS optional for decentralization).
- **Blockchain Layer**: Stellar's Soroban smart contracts (Rust) handle escrow logic: create, fund, submit work, and release/timeout funds. Stellar ledger stores transactions; Horizon API queries state.
- **Storage**: IPFS (via Pinata) for job descriptions and work proof (e.g., file hashes), ensuring decentralized data.
- **User Auth**: Freighter wallet for Stellar account management and transaction signing.

### Flow
1. Users connects their Stellar wallet and access a matching platform for pacting the services to be done.
2. Freelancer stablish services conditions and the corresponding employer accept or reget it.
3. The contract implement Blend to generate yield with the locked funds.
3. When employers approves, the contract release the funds for the corresponding milestone.
4. Payments settle instantly and are transparently viewable on Stellar Explorer.
5. In case of disagreement users can request for a decentralized intermediation, based on game's theory and the Neural Quorum Governance system, assuring high incentives.  

## Stellar Components Used
- **Stellar SDK**: Integrates frontend with Stellar for account management, payments, and contract calls.
- **Soroban**: Rust-based smart contracts for escrow (create, fund, release, timeout). Deployed on Futurenet testnet.
- **Horizon API**: Queries ledger for transaction history and account balances.
- **Freighter Wallet**: Browser extension for user authentication and transaction signing.
- **Stellar Assets**: USDC for stable payments, XLM for native transactions. Trustlines ensure secure asset handling.
- **Stellar Explorer**: Displays transparent transaction logs for demo.

## Installation Steps
### Prerequisites
- Node.js (v16+)
- Rust (for Soroban contracts)
- Freighter wallet (browser extension)
- Git, npm, and a modern browser

...


## Deployment

### Prerequisites

- Rust toolchain with `wasm32-unknown-unknown` target
- Soroban CLI
- Stellar network access (testnet/mainnet)

### Build, deploy and execute

Inside main folder give permission to script files for being executed with:
```bash
chmod +x ./accounts.sh
chmod +x ./run.sh
```

Execute accounts.sh only once running
```bash
./accounts.sh
```
Execute run.sh only once runningto build, optimize and execute transations
```bash
./run.sh
```
### Tests

For running test execute
```bash
cargo test
```

## Security Features

- **Access Control**: Only admin and authorized crowdfunding contract can mint
- **Proof Uniqueness**: Prevents duplicate NFTs for the same proof
- **Validation System**: Two-step process (creation + validation)
- **Immutable Records**: NFT metadata provides permanent milestone records

## Testing

Run the test suite:

```bash
cargo test
```

Key test scenarios:
- Contract initialization
- Milestone minting and validation
- Token transfers and approvals
- Campaign progress tracking
- Unauthorized access prevention

## Roadmap

- [x] Develop MVP with main functionalities 
- [x] Develop basic front-end to showcase user interactions 
- [x] Blend functions for deposit funds and generate yield

- [ ] Evaluate and implement a minimal contract balance to assure liquidity for instant user payments
- [ ] Implement an on-chain reputation system
- [ ] Create registry platform for user verification, offering an optional 'checked' status
- [ ] Create a matching page where users can search and chat before before agreeing to a service
- [ ] Implement premium client tools like talent matching, project management features, or hiring support
- [ ] Develop dashboard for users to manage and track their services and disputes
- [ ] Create analytics system for tracking usage metrics
- [ ] Create mobile app for allowing interactions on smartphones
- [ ] Create NFT smart contract to reward users after reaching certain achievements
- [ ] Implement back-end service for managing no fundamental information off-chain
- [ ] Enhance Stellar SDK integration for broader blockchain interactions
- [ ] Create documentation and video tutorials for better user boarding 
- [ ] Implement support for multiple Stellar-compatible wallet providers
- [ ] Implement governance system for community-driven protocol upgrades
