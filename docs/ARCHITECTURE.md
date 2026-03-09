# Solana Stablecoin Standard - Technical Architecture

This document describes the three-layer architecture of the Solana Stablecoin Standard (SSS).

## Layer 1: Base SDK
Provides the primitives for token creation using Token-2022. It handles the deployment of the core mint and metadata.

## Layer 2: Modules
Composable extensions that add specific functionality:
- **Compliance:** Transfer hooks and Blacklist PDAs.
- **Roles:** Fine-grained RBAC for minting and seizing.

## Layer 3: Standard Presets
Opinionated configurations for rapid deployment:
- **SSS-1 (Minimal):** No compliance overhead, standard stablecoin behavior.
- **SSS-2 (Compliant):** Built-in blacklist and permanent delegate for regulatory compliance.

## Security Model
SSS uses a multi-authority model. Separate keys can be assigned to:
- Minter role (with optional quotas)
- Blacklister role
- Pauser role
- Master authority
