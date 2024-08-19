import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Cega, IDL } from "../target/types/cega";
import * as bs58 from "bs58";
import { BN } from "@coral-xyz/anchor";
import { PublicKey, Commitment, Keypair, SystemProgram } from "@solana/web3.js";
import { randomBytes } from "crypto";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID as associatedTokenProgram,
  TOKEN_PROGRAM_ID as tokenProgram,
  createMint,
  createAccount,
  mintTo,
  getAssociatedTokenAddress,
  TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID
} from "@solana/spl-token"
const commitment: Commitment = "confirmed"; 

describe("cega solana assignment", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const programId = new PublicKey(
    "8YapqrY61XUSMUw6bCNK9kogG65mxM2j8PpRw4jaf8Vx"
  );
  const program = new anchor.Program<Cega>(
    IDL,
    programId,
    anchor.getProvider()
  );

  const seller = Keypair.fromSecretKey(bs58.decode(wallet));
  const buyer = Keypair.fromSecretKey(bs58.decode(wallet_two))
  // Random seed
  const seed = new BN(randomBytes(8));

  // PDAs
  const auth = PublicKey.findProgramAddressSync(
    [Buffer.from("auth")],
    program.programId
  )[0];
  const config = PublicKey.findProgramAddressSync(
    [Buffer.from("config"), seed.toBuffer().reverse()],
    program.programId
  )[0];

  // Mints
  let mint_x: PublicKey;
  let mint_usdc: PublicKey;

  // ATAs
  let seller_x_ata: PublicKey;
  let seller_usdc_ata: PublicKey;
  let buyer_x_ata: PublicKey;
  let buyer_usdc_ata: PublicKey;
  let vault_x_ata: PublicKey;

  it("Create mints, tokens and ATAs", async () => {
    mint_x = new PublicKey("Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr")
    mint_usdc = new PublicKey("")
    seller_x_ata = await getAssociatedTokenAddress(mint_x, seller.publicKey, false, tokenProgram)
    seller_usdc_ata = await getAssociatedTokenAddress(mint_usdc, seller.publicKey, false, tokenProgram);
    buyer_x_ata = await getAssociatedTokenAddress(mint_x, buyer.publicKey, false, tokenProgram);
    buyer_usdc_ata = await getAssociatedTokenAddress(mint_usdc, buyer.publicKey, false, tokenProgram);
    // Create take ATAs
    vault_x_ata = await getAssociatedTokenAddress(mint_x, auth, true, tokenProgram);

})

  const confirmTx = async (signature: string) => {
    const latestBlockhash = await anchor
      .getProvider()
      .connection.getLatestBlockhash();
    await anchor.getProvider().connection.confirmTransaction(
      {
        signature,
        ...latestBlockhash,
      },
      commitment
    );
  };

  const confirmTxs = async (signatures: string[]) => {
    await Promise.all(signatures.map(confirmTx));
  };
});
