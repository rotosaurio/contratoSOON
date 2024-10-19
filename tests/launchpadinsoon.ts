import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Launchpadinsoon } from "../target/types/launchpadinsoon";
import { expect } from "chai";
import * as dotenv from 'dotenv';
dotenv.config();

describe("launchpadinsoon", () => {
  const provider = new anchor.AnchorProvider(new anchor.web3.Connection(process.env.ANCHOR_PROVIDER_URL!, 'confirmed', { commitment: 'confirmed', confirmTransactionInitialTimeout: 60000 }), anchor.AnchorProvider.env().wallet, {});

  anchor.setProvider(provider);

  const program = anchor.workspace.Launchpadinsoon as Program<Launchpadinsoon>;

  let sale: anchor.web3.Keypair;
  let admin: anchor.web3.Keypair;
  let user: anchor.web3.Keypair;
  let tokenMint: anchor.web3.Keypair;
  let tokenVault: anchor.web3.PublicKey;
  let buyerTokenAccount: anchor.web3.PublicKey;
  let treasury: anchor.web3.Keypair;

  before(async () => {
    admin = anchor.web3.Keypair.generate();
    user = anchor.web3.Keypair.generate();
    tokenMint = anchor.web3.Keypair.generate();
    treasury = anchor.web3.Keypair.generate();

    // Airdrop SOL to admin and user
    await provider.connection.requestAirdrop(admin.publicKey, 10 * anchor.web3.LAMPORTS_PER_SOL);
    await provider.connection.requestAirdrop(user.publicKey, 10 * anchor.web3.LAMPORTS_PER_SOL);

    // Create token mint and accounts
    await anchor.utils.token.createMint(provider, admin, admin.publicKey, null, 9, tokenMint);
    tokenVault = await anchor.utils.token.createAssociatedTokenAccount(provider, admin, tokenMint.publicKey, provider.wallet.publicKey);
    buyerTokenAccount = await anchor.utils.token.createAssociatedTokenAccount(provider, user, tokenMint.publicKey, user.publicKey);
  });

  it("Initializes the sale", async () => {
    sale = anchor.web3.Keypair.generate();
    const totalTokens = new anchor.BN(1000000);
    const price = new anchor.BN(10);

    await program.methods.initialize(totalTokens, price)
      .accounts({
        sale: sale.publicKey,
        admin: admin.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([sale, admin])
      .rpc();

    const saleAccount = await program.account.sale.fetch(sale.publicKey);
    expect(saleAccount.totalTokens.toNumber()).to.equal(1000000);
    expect(saleAccount.price.toNumber()).to.equal(10);
    expect(saleAccount.paused).to.be.false;
  });

  it("Adds a user to the whitelist", async () => {
    await program.methods.addToWhitelist(user.publicKey)
      .accounts({
        sale: sale.publicKey,
        admin: admin.publicKey,
      })
      .signers([admin])
      .rpc();

    const saleAccount = await program.account.sale.fetch(sale.publicKey);
    expect(saleAccount.whitelist).to.include(user.publicKey);
  });

  it("Sets allocation for a user", async () => {
    const allocation = new anchor.BN(1000);

    await program.methods.setAllocation(user.publicKey, allocation)
      .accounts({
        sale: sale.publicKey,
        admin: admin.publicKey,
      })
      .signers([admin])
      .rpc();

    const saleAccount = await program.account.sale.fetch(sale.publicKey);
    const userAllocation = saleAccount.allocations.find(([pubkey, _]) => pubkey.equals(user.publicKey));
    expect(userAllocation[1].toNumber()).to.equal(1000);
  });

  it("Buys tokens", async () => {
    const amount = new anchor.BN(500);

    await program.methods.buyTokens(amount)
      .accounts({
        sale: sale.publicKey,
        buyer: user.publicKey,
        buyerTokenAccount: buyerTokenAccount,
        tokenVault: tokenVault,
        treasury: treasury.publicKey,
        saleAuthority: provider.wallet.publicKey,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      })
      .signers([user])
      .rpc();

    const saleAccount = await program.account.sale.fetch(sale.publicKey);
    expect(saleAccount.tokensSold.toNumber()).to.equal(500);

    const buyerTokenBalance = await provider.connection.getTokenAccountBalance(buyerTokenAccount);
    expect(buyerTokenBalance.value.uiAmount).to.equal(500);
  });

  it("Pauses and unpauses the sale", async () => {
    await program.methods.pauseSale()
      .accounts({
        sale: sale.publicKey,
        admin: admin.publicKey,
      })
      .signers([admin])
      .rpc();

    let saleAccount = await program.account.sale.fetch(sale.publicKey);
    expect(saleAccount.paused).to.be.true;

    await program.methods.unpauseSale()
      .accounts({
        sale: sale.publicKey,
        admin: admin.publicKey,
      })
      .signers([admin])
      .rpc();

    saleAccount = await program.account.sale.fetch(sale.publicKey);
    expect(saleAccount.paused).to.be.false;
  });

  it("Creates a vesting schedule", async () => {
    const amount = new anchor.BN(1000);
    const releaseTime = new anchor.BN(Math.floor(Date.now() / 1000) + 3600); // 1 hour from now

    await program.methods.createVesting(amount, releaseTime)
      .accounts({
        sale: sale.publicKey,
        admin: admin.publicKey,
        user: user.publicKey,
      })
      .signers([admin, user])
      .rpc();

    const saleAccount = await program.account.sale.fetch(sale.publicKey);
    const userVesting = saleAccount.vestings.find(([pubkey, _]) => pubkey.equals(user.publicKey));
    expect(userVesting[1].amount.toNumber()).to.equal(1000);
    expect(userVesting[1].releaseTime.toNumber()).to.equal(releaseTime.toNumber());
    expect(userVesting[1].claimed).to.be.false;
  });

  it("Updates sale parameters", async () => {
    const newPrice = new anchor.BN(15);
    const newTotalTokens = new anchor.BN(2000000);

    await program.methods.updateParameters(newPrice, newTotalTokens)
      .accounts({
        sale: sale.publicKey,
        admin: admin.publicKey,
      })
      .signers([admin])
      .rpc();

    const saleAccount = await program.account.sale.fetch(sale.publicKey);
    expect(saleAccount.price.toNumber()).to.equal(15);
    expect(saleAccount.totalTokens.toNumber()).to.equal(2000000);
  });
});
