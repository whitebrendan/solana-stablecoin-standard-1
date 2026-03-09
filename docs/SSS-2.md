# SSS-2: Compliant Stablecoin Standard

## Overview
SSS-2 is an opinionated preset for regulated stablecoins on Solana. It leverages Token-2022 extensions to provide on-chain compliance enforcement.

## Extensions Enabled
1. **Permanent Delegate:** Allows the issuer to seize tokens from any account (e.g., for law enforcement requests).
2. **Transfer Hook:** Executes custom logic on every transfer to ensure compliance.
3. **Metadata Pointer:** Links the token to its official metadata.

## Compliance Logic
Every transfer is checked against an on-chain **Blacklist**. 
- If the `source` or `destination` is in the blacklist, the transfer is blocked.
- Blacklist is managed by a designated `Blacklister` role.

## Roles
- **Master Authority:** Can change roles and seize funds.
- **Minter:** Can issue new tokens.
- **Blacklister:** Can add/remove addresses from the compliance list.
- **Pauser:** Can freeze all protocol interactions.
