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
- `programs/`: Anchor programs (SSS Core & Transfer Hook).
- `sdk/`: TypeScript SDK and Admin CLI.
- `backend/`: Indexer and Mint/Burn services.
- `docs/`: Technical specifications and operator guides.

## Quick Start
```bash
npm install @stbr/sss-token
sss-token init --preset sss-1 --name "MyStable" --symbol "MST"
```
