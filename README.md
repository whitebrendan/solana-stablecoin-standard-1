# SoroSave (Legacy)
# Solana Stablecoin Standard (SSS)

This repository contains the core standards, SDK, and on-chain programs for deploying and managing stablecoins on Solana using Token-2022 extensions.

## Standards

### SSS-1: Minimal Stablecoin
- **Features:** Mint Authority, Freeze Authority, Metadata.
- **Use Case:** Internal tokens, ecosystem settlement.

### SSS-2: Compliant Stablecoin
- **Features:** SSS-1 + Permanent Delegate + Transfer Hook + Blacklist.
- **Use Case:** Regulated assets (USDC/USDT class).

## Structure
- `programs/sss-token/`: Core logic for minting, roles, and blacklisting.
- `programs/sss-transfer-hook/`: Transfer hook for mandatory blacklist enforcement.
- `sdk/core/`: TypeScript library for protocol interaction.
- `sdk/cli/`: Admin command line tool.
- `backend/service/`: API for coordinating mint/burn and compliance.
- `docs/`: ARCHITECTURE.md, OPERATIONS.md, SSS-1.md, SSS-2.md.

## Quick Start
```bash
# 1. Initialize stablecoin with SSS-2 (Compliant) preset
sss-token init --preset sss-2 --name "EuroStable" --symbol "EURS"

# 2. Add an address to blacklist
sss-token blacklist add <BAD_ACTOR_PUBKEY> --reason "Suspicious activity"

# 3. Seize funds from blacklisted account to treasury
sss-token seize <BAD_ACTOR_PUBKEY> --to <TREASURY_PUBKEY> --amount 1000
```

## Built for Superteam Brazil
Developed as part of the open-source Solana Stablecoin Standard initiative.
