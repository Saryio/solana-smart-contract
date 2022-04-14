const anchor = require('@project-serum/anchor');
const { SystemProgram } = require('@solana/web3.js');

const main = async() => {
  console.log("🚀 Starting test...")
  

  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);

  const program = anchor.workspace.Myepicproject;
  
  const baseAccount = anchor.web3.Keypair.generate()

  const tx = await program.rpc.initialize({
    accounts:{
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId
    },
    signers: [baseAccount]
  });

  console.log("📝 Your transaction signature", tx);

  let account = await program.account.baseAccount.fetch(baseAccount.publicKey)
  console.log(`GIF Count ${account.totalGifs.toString()}`)

  await program.rpc.addGif("https://ipfs.io/ipfs/QmaQz5myZCgoY6KbtYFq3YjmN52ye2TDdwortrGenSNEn1?filename=um%20maluco%20odeia%20o%20chris.png", "Antony",  {
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey
    }
  })

  account = await program.account.baseAccount.fetch(baseAccount.publicKey)
  console.log(`GIF Count ${account.totalGifs.toString()}`)

  console.log(`GIF List`, account.gifList)
}

const runMain = async () => {
  try {
    await main();
    process.exit(0);
  } catch (error) {
    console.error(error);
    process.exit(1);
  }
};

runMain();