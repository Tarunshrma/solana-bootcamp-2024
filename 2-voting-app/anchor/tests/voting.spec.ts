import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { PublicKey } from '@solana/web3.js';
import { Voting } from 'anchor/target/types/Voting';
import { assert } from 'chai';

describe('Voting', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Voting as Program<Voting>;

  it('initializePoll', async () => {

    const tx = await program.methods.initilizePoll(
        new anchor.BN(1),
        "test-poll",
        "description",
        new anchor.BN(0),
        new anchor.BN(1759508293),
        new anchor.BN(0),
    )
    .rpc();

    console.log('Your transaction signature', tx);

    const [pollAddress] = PublicKey.findProgramAddressSync(
        [Buffer.from("poll"), new anchor.BN(1).toArrayLike(Buffer, "le", 8)],
        program.programId
      );
    

    const poll = await program.account.pollAccount.fetch(pollAddress);
    console.log('{#?}', poll);

    assert.equal(poll.pollId.toNumber(), 1);

  });

});