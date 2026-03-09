import { Connection, PublicKey, Keypair } from "@solana/web3.js";
import { Program, AnchorProvider, Idl } from "@coral-xyz/anchor";

export enum Presets {
    SSS_1 = "sss-1",
    SSS_2 = "sss-2"
}

export interface StablecoinOptions {
    preset?: Presets;
    name: String;
    symbol: String;
    decimals: number;
    authority: Keypair;
}

export class SolanaStablecoin {
    constructor(
        public connection: Connection,
        public programId: PublicKey,
        public config: any
    ) {}

    static async create(connection: Connection, options: StablecoinOptions) {
        // Logic to initialize the program state based on preset
        console.log(`Initializing stablecoin with preset: ${options.preset || 'custom'}`);
        return new SolanaStablecoin(connection, new PublicKey("SSS1111111111111111111111111111111111111111"), options);
    }

    async mint(recipient: PublicKey, amount: number) {
        console.log(`Minting ${amount} to ${recipient.toBase58()}`);
    }

    async blacklistAdd(address: PublicKey, reason: string) {
        console.log(`Blacklisting ${address.toBase58()} for reason: ${reason}`);
    }
}
