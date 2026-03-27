<div align="center">

<img src="https://img.shields.io/badge/Built%20on-Stellar-7c5cfc?style=for-the-badge&logo=stellar&logoColor=white" />
<img src="https://img.shields.io/badge/Smart%20Contract-Soroban-f5c842?style=for-the-badge" />
<img src="https://img.shields.io/badge/Wallet-Freighter-34d399?style=for-the-badge" />
<img src="https://img.shields.io/badge/License-MIT-fb7185?style=for-the-badge" />
<img src="https://img.shields.io/badge/Status-Live%20on%20Testnet-34d399?style=for-the-badge" />
![Contract Deployed](https://github.com/user-attachments/assets/7946a3b4-1155-47d9-8f81-e12e2d5c02c2)


<br/><br/>

```
  _                      _   _         _____
 | |    ___  _   _  __ _| | | |_ _   _|  ___|__  _ __ __ _  ___
 | |   / _ \| | | |/ _` | | | __| | | | |_ / _ \| '__/ _` |/ _ \
 | |__| (_) | |_| | (_| | | | |_| |_| |  _| (_) | | | (_| |  __/
 |_____\___/ \__, |\__,_|_|  \__|\__, |_|  \___/|_|  \__, |\___|
             |___/               |___/                |___/
```

### 🔥 Permissionless NFT Loyalty Programs on Stellar

*Mint stamps. Earn badges. Forge rewards. All on-chain.*

<br/>

[**Live Demo**](#) · [**Contract Docs**](#-contract-functions) · [**Deploy Guide**](#-deploy-the-contract) · [**Freighter**](https://freighter.app)

<br/>

> 🟢 **Contract Live on Stellar Testnet**
> `CAVL3F7R6A4WCHKDVVNCFGTCGJAUVR7766QRLZXZG2YZVOUOLEAOVFNT`
> [View on Stellar Expert ↗](https://stellar.expert/explorer/testnet/contract/CAVL3F7R6A4WCHKDVVNCFGTCGJAUVR7766QRLZXZG2YZVOUOLEAOVFNT)

</div>

---

## What is LoyaltyForge?

**LoyaltyForge** is a fully permissionless NFT loyalty platform built on **Stellar Soroban**. Any merchant, brand, or individual can launch a loyalty program in seconds — and any user can mint, collect, redeem, and trade loyalty NFTs without any admin approval or gatekeeping.

No owner keys. No whitelists. No middlemen. Just pure on-chain loyalty.

```
Merchant creates program  →  Customer mints NFT  →  Check-in earns stamps + points
       ↓                                                        ↓
Tier badge auto-upgrades  ←  Redeem reward  ←  Reach stamp threshold
```

---

## 📸 Contract Deployed

![Contract on Stellar Expert](assets/screenshots/contract-deployed.png)

> Contract `CAVL...VFNT` live on Stellar Testnet — created 2026-03-27, WASM verified.

---

## ✦ Features at a Glance

| Feature | Description | Permissionless? |
|---|---|:---:|
| 🏪 **Create Programs** | Any wallet deploys a loyalty program on-chain | ✅ Anyone |
| 🎫 **Mint Loyalty NFTs** | Users mint their own NFT badge for any program | ✅ Anyone |
| 📍 **Stamp Cards** | Check-in to earn stamps; reach threshold to unlock reward | NFT owner |
| 🏅 **Tiered Badges** | Auto-upgrades on-chain: Bronze → Silver → Gold | Auto |
| ⭐ **Points System** | Earn configurable points per visit, accumulate over time | Auto |
| 🎁 **Redeem Rewards** | Spend stamps to claim reward, counter resets | NFT owner |
| 🔥 **Burn for Discount** | Destroy NFT (100+ pts) for a one-time discount code | NFT owner |
| ↗️ **Transfer NFTs** | Send loyalty NFT peer-to-peer, no approval needed | NFT owner |
| 👛 **Freighter Wallet** | Native Freighter browser extension integration | — |

---

## 🏗 Project Structure

```
loyaltyforge/
│
├── 📁 contracts/
│   └── 📁 loyalty/
│       ├── Cargo.toml              ← Soroban contract manifest
│       └── 📁 src/
│           ├── lib.rs              ← Main contract (programs, NFTs, tiers)
│           └── test.rs             ← Unit tests
│
├── 📁 app/                         ← Frontend (Next.js / Bun)
│   ├── 📁 components/
│   ├── package.json
│   └── tsconfig.json
│
├── loyaltyforge-dapp.html          ← Standalone single-file frontend
├── Makefile                        ← Build & deploy shortcuts
├── .gitignore
└── README.md
```

---

## 🚀 Deploy the Contract

### Prerequisites

```bash
# 1. Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. Add WASM compilation target
rustup target add wasm32-unknown-unknown

# 3. Install Stellar CLI
cargo install --locked stellar-cli --features opt
```

### Step 1 — Build the Contract

```bash
cd contracts/loyalty
cargo build --target wasm32-unknown-unknown --release
```

> Output: `target/wasm32-unknown-unknown/release/loyalty_contract.wasm`

### Step 2 — Fund a Testnet Identity

```bash
stellar keys generate --global deployer --network testnet
stellar keys fund deployer --network testnet
```

### Step 3 — Deploy to Testnet

```bash
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/loyalty_contract.wasm \
  --source deployer \
  --network testnet
```

> 📋 Copy the returned **Contract ID** — it starts with `C...`

### Step 4 — Wire the Frontend

Open `loyaltyforge-dapp.html` and update:

```js
const CONTRACT_ID = 'CAVL3F7R6A4WCHKDVVNCFGTCGJAUVR7766QRLZXZG2YZVOUOLEAOVFNT';
```

---

## 📡 Contract Functions

### 🏪 Programs

```bash
# Create a loyalty program — anyone can call this
stellar contract invoke \
  --id $CONTRACT_ID --source deployer --network testnet \
  -- create_program \
  --creator GBXXX... \
  --name "Cosmic Coffee" \
  --description "10 stamps = free specialty drink" \
  --brand_color "#f5c842" \
  --stamp_threshold 10 \
  --points_per_visit 15

# Query a program
stellar contract invoke --id $CONTRACT_ID --network testnet \
  -- get_program --program_id 1

# Total programs on-chain
stellar contract invoke --id $CONTRACT_ID --network testnet \
  -- get_program_count
```

### 🎫 NFTs

```bash
# Mint a loyalty NFT — anyone can mint for anyone
stellar contract invoke \
  --id $CONTRACT_ID --source deployer --network testnet \
  -- mint_loyalty_nft \
  --recipient GBXXX... \
  --program_id 1 \
  --metadata_uri "ipfs://QmXXX"

# Check in (earn stamp + points, auto tier upgrade)
stellar contract invoke \
  --id $CONTRACT_ID --source alice --network testnet \
  -- check_in \
  --owner GBXXX... \
  --token_id 1

# Redeem reward when stamps ≥ threshold
stellar contract invoke \
  --id $CONTRACT_ID --source alice --network testnet \
  -- redeem_reward \
  --owner GBXXX... \
  --token_id 1

# Burn NFT for a one-time discount (requires 100+ points)
stellar contract invoke \
  --id $CONTRACT_ID --source alice --network testnet \
  -- burn_for_discount \
  --owner GBXXX... \
  --token_id 1

# Transfer NFT peer-to-peer
stellar contract invoke \
  --id $CONTRACT_ID --source alice --network testnet \
  -- transfer \
  --from GBXXX... \
  --to GBYYY... \
  --token_id 1
```

### 🔍 Queries

```bash
# Get a single NFT
stellar contract invoke --id $CONTRACT_ID --network testnet -- get_nft --token_id 1

# All NFTs owned by an address
stellar contract invoke --id $CONTRACT_ID --network testnet -- get_owner_tokens --owner GBXXX...

# All NFTs in a program
stellar contract invoke --id $CONTRACT_ID --network testnet -- get_program_tokens --program_id 1

# Global counters
stellar contract invoke --id $CONTRACT_ID --network testnet -- get_token_count
stellar contract invoke --id $CONTRACT_ID --network testnet -- get_program_count
```

---

## 🧩 Tier System

Tiers are calculated **fully on-chain** during every `check_in` call — no manual upgrades, no admin intervention.

```
Visits  0 – 4   →   🔵  No Tier    (plain badge)
Visits  5 – 9   →   🥉  Bronze     (spinning bronze ring)
Visits 10 – 19  →   🥈  Silver     (spinning silver ring)
Visits 20+      →   🥇  Gold       (spinning gold ring)
```

Each tier upgrade emits an on-chain event that frontends can subscribe to.

---

## 🔐 Permissionless by Design

LoyaltyForge has **zero privileged roles**. The table below shows who can call what:

| Action | Auth Required | Open To |
|---|---|---|
| `create_program` | Caller signature | 🌍 Anyone |
| `mint_loyalty_nft` | None | 🌍 Anyone |
| `check_in` | NFT owner | Owner only |
| `redeem_reward` | NFT owner + enough stamps | Owner only |
| `burn_for_discount` | NFT owner + 100 pts | Owner only |
| `transfer` | NFT owner | Owner only |

> ⚠️ There is no `set_admin`, `pause`, `upgrade`, or `whitelist` function.
> The contract is immutable once deployed.

---

## 🌐 Run the Frontend Locally

The frontend is a **zero-dependency single HTML file** — no npm install, no build step needed.

```bash
# Option 1 — open directly in browser
open loyaltyforge-dapp.html

# Option 2 — serve with npx
npx serve .

# Option 3 — Python
python3 -m http.server 8080
```

Then:
1. Install [Freighter](https://freighter.app) browser extension
2. Switch Freighter network to **Testnet**
3. Fund your testnet wallet at [Stellar Laboratory](https://laboratory.stellar.org)
4. Open the app and click **Connect Freighter**

---

## ⚡ Makefile Shortcuts

```bash
make build      # Compile contract to WASM
make deploy     # Deploy to Stellar testnet
make test       # Run contract unit tests
make clean      # Clean build artifacts
```

---

## 🔗 Resources

| Resource | Link |
|---|---|
| Stellar Developer Docs | https://developers.stellar.org |
| Soroban Smart Contracts | https://soroban.stellar.org |
| Freighter Wallet | https://freighter.app |
| Stellar Testnet Horizon | https://horizon-testnet.stellar.org |
| Soroban Testnet RPC | https://soroban-testnet.stellar.org |
| Stellar Laboratory | https://laboratory.stellar.org |
| Stellar Discord | https://discord.gg/stellardev |

---

## 📄 License

MIT — free to use, fork, and build on.
![contract-deployed](https://github.com/user-attachments/assets/0d984bc1-ba7d-4529-8091-56795b78f39c)

---

<div align="center">

Built with 🔥 on **Stellar Soroban** · Powered by **Freighter** · Forged for Web3 Loyalty

</div>
