import { Connection, PublicKey, Keypair, Transaction, SystemProgram } from "@solana/web3.js";
import { 
    TOKEN_2022_PROGRAM_ID, 
    createInitializeMintInstruction, 
    getMintLen, 
    ExtensionType,
    createInitializePermanentDelegateInstruction,
    createInitializeTransferHookInstruction
} from "@solana/spl-token";

export enum Presets {
    SSS_1 = "sss-1",
    SSS_2 = "sss-2"
}

export interface StablecoinOptions {
    preset?: Presets;
    name: string;
    symbol: string;
    decimals: number;
    authority: Keypair;
}

export class SolanaStablecoin {
    constructor(
        public connection: Connection,
        public mint: PublicKey,
        public config: StablecoinOptions
    ) {}

    static async create(connection: Connection, options: StablecoinOptions) {
        const mintKeypair = Keypair.generate();
        const extensions: ExtensionType[] = [ExtensionType.MetadataPointer];
        
        if (options.preset === Presets.SSS_2) {
            extensions.push(ExtensionType.PermanentDelegate);
            extensions.push(ExtensionType.TransferHook);
        }

        const mintLen = getMintLen(extensions);
        const lamports = await connection.getMinimumBalanceForRentExemption(mintLen);

        const transaction = new Transaction().add(
            SystemProgram.createAccount({
                fromPubkey: options.authority.publicKey,
                newAccountPubkey: mintKeypair.publicKey,
                space: mintLen,
                lamports,
                programId: TOKEN_2022_PROGRAM_ID,
            })
        );

        if (options.preset === Presets.SSS_2) {
            transaction.add(
                createInitializePermanentDelegateInstruction(
                    mintKeypair.publicKey,
                    options.authority.publicKey,
                    TOKEN_2022_PROGRAM_ID
                )
            );
            // Add transfer hook initialization here
        }

        transaction.add(
            createInitializeMintInstruction(
                mintKeypair.publicKey,
                options.decimals,
                options.authority.publicKey,
                options.authority.publicKey,
                TOKEN_2022_PROGRAM_ID
            )
        );

        // Send and confirm...
        console.log(`Initialized ${options.preset} stablecoin: ${mintKeypair.publicKey.toBase58()}`);
        return new SolanaStablecoin(connection, mintKeypair.publicKey, options);
    }

    async mint(recipient: PublicKey, amount: number) {
        console.log(`[SDK] Minting ${amount} to ${recipient.toBase58()}`);
    }

    async blacklistAdd(address: PublicKey, reason: string) {
        if (this.config.preset !== Presets.SSS_2) {
            throw new Error("Blacklist only supported in SSS-2");
        }
        console.log(`[SDK] Blacklisting ${address.toBase58()} | Reason: ${reason}`);
    }
}
