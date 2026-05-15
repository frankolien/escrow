import * as anchor from "@coral-xyz/anchor";
import { BN, Program } from "@coral-xyz/anchor";
import { Escrow } from "../target/types/escrow";
import {
  TOKEN_PROGRAM_ID,
  createMint,
  createAssociatedTokenAccount,
  mintTo,
  getAccount,
  getAssociatedTokenAddressSync,
} from "@solana/spl-token";
import { PublicKey } from "@solana/web3.js";
import { assert } from "chai";

describe("escrow", () => {
  // 1. PROVIDER + PROGRAM
  // The provider bundles a connection (which cluster?) and a wallet
  // (who pays fees + signs?). `AnchorProvider.env()` reads both from
  // env vars set by `anchor test` — cluster from Anchor.toml, wallet
  // from ~/.config/solana/id.json.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // `anchor.workspace.escrow` is the typed client generated from
  // target/types/escrow.ts (rebuilt by `anchor build`).
  const program = anchor.workspace.escrow as Program<Escrow>;

  // The wallet that will play the role of "maker" in our offer.
  const maker = (provider.wallet as anchor.Wallet).payer;

  // We'll fill these in `before()`.
  let tokenMintA: PublicKey;
  let tokenMintB: PublicKey;
  let makerAtaA: PublicKey;

  // An arbitrary id so each test run produces a unique PDA.
  const offerId = new BN(Date.now());
  const offeredAmount = new BN(1_000_000); // 1 token at 6 decimals
  const wantedAmount = new BN(2_000_000);

  before(async () => {
    // 2. SET UP ON-CHAIN STATE
    // Anchor tests run against a fresh local validator (started by
    // `anchor test`), so we have to create everything from scratch.

    // Create two SPL mints. `createMint` returns the mint pubkey.
    // Args: connection, fee payer, mint authority, freeze authority, decimals.
    tokenMintA = await createMint(provider.connection, maker, maker.publicKey, null, 6);
    tokenMintB = await createMint(provider.connection, maker, maker.publicKey, null, 6);

    // Create the maker's ATA for token A and fund it. An ATA
    // (associated token account) is a deterministic PDA holding
    // a user's balance of a given mint.
    makerAtaA = await createAssociatedTokenAccount(
      provider.connection,
      maker,
      tokenMintA,
      maker.publicKey,
    );
    await mintTo(provider.connection, maker, tokenMintA, makerAtaA, maker, 10_000_000);
  });

  it("makes an offer", async () => {
    // 3. DERIVE PDAs
    // The program uses `seeds = [b"offer", maker, id]` to derive the
    // offer account. We mirror that off-chain so we know the address.
    const [offerPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("offer"),
        maker.publicKey.toBuffer(),
        offerId.toArrayLike(Buffer, "le", 8), // u64 little-endian
      ],
      program.programId,
    );

    // The vault is the ATA owned by the offer PDA. `getAssociatedTokenAddressSync`
    // computes the address without hitting the network — useful when the
    // account doesn't exist yet (our program will create it).
    // `allowOwnerOffCurve = true` because the offer PDA is not a valid ed25519 point.
    const vault = getAssociatedTokenAddressSync(tokenMintA, offerPda, true);

    // 4. CALL THE INSTRUCTION
    // `program.methods.<ixName>(args).accounts({...}).rpc()` is the
    // standard pattern. In Anchor 0.31, `.accounts()` only accepts
    // accounts the client can't derive on its own — the maker's ATA
    // and the vault ATA are auto-resolved from the IDL's seed info,
    // so we don't pass them. Use `.accountsPartial({...})` if you
    // ever need to override a derived address explicitly.
    const tx = await program.methods
      .makeOffer(offerId, offeredAmount, wantedAmount)
      .accounts({
        maker: maker.publicKey,
        tokenMintA,
        tokenMintB,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();

    console.log("make_offer tx:", tx);

    // 5. ASSERT
    // After the call, the vault should hold the offered amount and
    // the Offer account should store the parameters we passed in.

    const vaultAccount = await getAccount(provider.connection, vault);
    assert.strictEqual(vaultAccount.amount.toString(), offeredAmount.toString());

    // `program.account.offer.fetch(pda)` deserializes the on-chain
    // account using the IDL — the resulting object has the same field
    // names as the Rust `Offer` struct.
    const offer = await program.account.offer.fetch(offerPda);
    assert.strictEqual(offer.id.toString(), offerId.toString());
    assert.ok(offer.maker.equals(maker.publicKey));
    assert.ok(offer.tokenMintA.equals(tokenMintA));
    assert.ok(offer.tokenMintB.equals(tokenMintB));
    assert.strictEqual(offer.tokenBWantedAmount.toString(), wantedAmount.toString());
  });
});
