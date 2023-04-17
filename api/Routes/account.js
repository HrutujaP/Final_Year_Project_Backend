import express from 'express';
import { Actor,} from '@dfinity/agent';
import {Principal } from '@dfinity/principal';
import { idlFactory } from '../Canisters/account/Account.did.js';
import { agent } from '../agent.js';


var router = express.Router();

// const accountActor = Actor.createActor(idlFactory, { agent, canisterId: Principal.fromText('ryjl3-tyaaa-aaaaa-aaaba-cai') });
const accountActor = Actor.createActor(idlFactory, { agent, canisterId: Principal.fromText('yvrmz-eaaaa-aaaao-aifqq-cai') });

router.post('/create_account', async (req, res) => {
    
    try{

        const { name, email } = req.body;
        const result = await accountActor.create_account(String(name), String(email));
        // console.log(result);
        res.send(Principal.fromUint8Array(result[0]._arr).toText());
    }
    catch(err){
        console.log(err);
        res.send(err);
    }
});

router.get('/get_accounts', async (req, res) => {
    try{

        const result = await accountActor.get_all_accounts();
        const accounts = await Promise.all(
        result.map(async (account) => {
            const principal = Principal.fromUint8Array(account._arr);
            const str_principal = principal.toText();
            // console.log(principal.toString());
            const holder_name = await accountActor.get_account(principal);
            return {Principal:str_principal, holder_name: holder_name[0]};
        })
        );
        res.send(accounts);
    }
    catch(err){
        console.log(err);
        res.send(err);
    }
});

export default router;