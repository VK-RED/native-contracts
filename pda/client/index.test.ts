import { Connection, Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction } from "@solana/web3.js"
import {describe, it, expect } from "bun:test"
import { Favorites, FavoritesInstruction, InstructionType } from "./utils";

const keypair = Keypair.generate();
// NOTE: REPLACE THIS WITH YOUR DEPLOYED PROGRAM ID
const PROGRAM_ID = new PublicKey("BGyYwZUKfZjhfbjmaiymNfSX5vxdvxZU5C9BmtYL4Vyp");
const connection = new Connection('http://localhost:8899');

describe("Favorites Program", async () => {

    const seeds = [Buffer.from("favorites"), keypair.publicKey.toBuffer()];
    const [favoritesAccount] = PublicKey.findProgramAddressSync(seeds, PROGRAM_ID);

    console.log("USER PUBKEY", keypair.publicKey.toBase58());
    console.log("FAVORITES PUBKEY", favoritesAccount.toBase58());

    console.log("AIRDROPPING 5 SOL")
    const airdropSig = await connection.requestAirdrop(keypair.publicKey, 5 * LAMPORTS_PER_SOL);
    const {blockhash, lastValidBlockHeight} = (await connection.getLatestBlockhash());
    await connection.confirmTransaction({blockhash, signature:airdropSig, lastValidBlockHeight }, 'finalized');
    console.log("DONE AIRDROPPING")

    it("should initialize pda account", async () => {

        const initInstruction = new FavoritesInstruction(InstructionType.InitFavorites, []);
        const serializedIxData = initInstruction.getSerializedIxDataForInitFavorites();

        const tx = new Transaction().add({
            keys:[
                {isSigner: true, isWritable:false, pubkey: keypair.publicKey},
                {isSigner:false, isWritable:true, pubkey:favoritesAccount},
                {isSigner: false, isWritable:false, pubkey:SystemProgram.programId}
            ],
            programId: PROGRAM_ID,
            data:Buffer.from(serializedIxData)
        });

        const txSig = await connection.sendTransaction(tx, [keypair]);

        console.log("The Intialize tx Signature", txSig);
        console.log("Waiting for the Transaction Confirmation");

        const {blockhash, lastValidBlockHeight} = (await connection.getLatestBlockhash());
        await connection.confirmTransaction({blockhash, signature:txSig, lastValidBlockHeight }, 'finalized');

        const onChainFavoritesAccount = await connection.getParsedAccountInfo(favoritesAccount);
        const serializedData = onChainFavoritesAccount.value?.data as Buffer<ArrayBufferLike>;

        const favorites = Favorites.getDeserializedData(serializedData);

        console.log("Favorites Stored in Chain");
        console.log(favorites);

        expect(JSON.stringify(favorites.data)).toBe(JSON.stringify([]));
    })

    it("Should set Favorites", async() => {

        const favoritesToSet : (string|null)[] = ["random", "sub", "come", "here", "solana"];

        const setFavsIx = new FavoritesInstruction(InstructionType.SetFavorites, favoritesToSet);
        const serializedData = setFavsIx.getSerializedIxDataForSetFavorites();

        const setFavsTx = new Transaction().add({
            keys:[
                {pubkey: keypair.publicKey, isSigner:true, isWritable:false},
                {pubkey: favoritesAccount, isSigner: false, isWritable:true}
            ],
            programId: PROGRAM_ID,
            data: Buffer.from(serializedData),
        })

        const txId = await connection.sendTransaction(setFavsTx, [keypair]);
        console.log("Set Favorites Transaction Signature", txId);

        console.log("Waiting for the tx confirmation");

        const {blockhash, lastValidBlockHeight} = (await connection.getLatestBlockhash());
        await connection.confirmTransaction({blockhash, signature:txId, lastValidBlockHeight }, 'finalized');

        console.log("On Chain Favorites Result");
        const onChainFavoritesAccount = await connection.getParsedAccountInfo(favoritesAccount);
        const serialized = onChainFavoritesAccount.value?.data as Buffer<ArrayBufferLike>;

        const onChainFavorites = Favorites.getDeserializedData(serialized);

        console.log("Favorites Stored in Chain");
        console.log(onChainFavorites);

        expect(JSON.stringify(onChainFavorites.data)).toBe(JSON.stringify(favoritesToSet));

    })

})


