import express from 'express';
import { Actor,} from '@dfinity/agent';
import {Principal } from '@dfinity/principal';
import { idlFactory } from '../Canisters/account/Account.did.js';
import { agent } from '../agent.js';


var router = express.Router();

// const accountActor = Actor.createActor(idlFactory, { agent, canisterId: Principal.fromText('rrkah-fqaaa-aaaaa-aaaaq-cai') });
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
            var holder_name = await accountActor.get_account(principal);
            holder_name = holder_name[0].Name;
            return {Principal:str_principal, holder_name: holder_name};
        })
        );
        res.send(accounts);
    }
    catch(err){
        console.log(err);
        res.send(err);
    }
});


router.get('/get_account',async (req, res) => {
    try{
        const account_principal= req.query.account_principal.toString();
        const principal = Principal.fromText(account_principal);
        var result = await accountActor.get_account(principal);
        result = result[0];

        if(result == null){
            res.send("Account not found");
            return;
        }

        var rented_storages = result.Rented_Storages;
        var my_storages = result.My_Storages;

        rented_storages = rented_storages.map((storage) => {
            const principal = Principal.fromUint8Array(storage._arr);
            return principal.toText();
            
        });
        
        my_storages = my_storages.map((storage) => {
            const principal = Principal.fromUint8Array(storage._arr);
            return principal.toText();
        });

        result['Rented_Storages'] = rented_storages;
        result['My_Storages'] = my_storages;
        result["Id"] = account_principal

        result = JSON.parse(JSON.stringify(result, (key, value) =>
        typeof value === 'bigint'
            ? value.toString()
            : value // return everything else unchanged
        ));

        res.send(result);

    }catch(err){
        console.log(err);
        res.send(err);
    }

});

router.get('/get_storages', async (req, res) => {
    try {
        const  account_principal = req.query.account_principal.toString();
        const principal = Principal.fromText(account_principal); 
        const [result] = await accountActor.get_all_storages(principal); 
        const rented_storages = result.Rented_Storages.map(storage => Principal.fromUint8Array(storage._arr).toText()); 
        const my_storages = result.My_Storages.map(storage => Principal.fromUint8Array(storage._arr).toText()); 
        res.send({ ...result, Rented_Storages: rented_storages, My_Storages: my_storages }); 
    } catch (err) {
        console.log(err); 
        res.send(err); 
    } 
});


router.post('/add_balance', async (req, res) => {
    try{
        const { account_principal, amount } = req.body;
        const principal = Principal.fromText(account_principal);
        const result = await accountActor.add_balance(principal, BigInt(amount));
        console.log(result);
        res.send({updated_balance:result.toString()});

    }catch(err){
        console.log(err);
        res.send(err);
    }
});


router.post('/withdraw_balance', async (req, res) => {
    try{
        const { account_principal, amount } = req.body;
        const principal = Principal.fromText(account_principal);
        var result = await accountActor.withdraw_balance(principal, BigInt(amount));
        result = result[0];
        // console.log(result);
        if(result != null){

            res.send({updated_balance:result.toString()});
        }else{
            res.send({updated_balance:"Account not found"});
        }

    }catch(err){
        console.log(err);
        res.send(err);
    }
});



router.get('/get_storage',async (req, res) => {
    try{
        const storage_principal = req.query.storage_principal.toString();
        const principal = Principal.fromText(storage_principal);
        var result = await accountActor.get_storage(principal);
        result = result[0];

        result["Id"] = storage_principal
        var owener_principal = Principal.fromUint8Array(result.OwnerPrincipal._arr);
        console.log(owener_principal);
        result["OwnerPrincipal"] = owener_principal.toText();
        var renter = result["RenterPrincipal"];
        renter = renter[0];
        if(renter != null){
            var renter_principal = Principal.fromUint8Array(renter._arr);
            result["RenterPrincipal"] = renter_principal.toText();
        }

        result = JSON.parse(JSON.stringify(result, (key, value) =>
        typeof value === 'bigint'
            ? value.toString()
            : value // return everything else unchanged
        ));

        res.send(result);
    }catch (err){
        console.log(err);
        res.send(err);
    }
});



router.post('/create_storage', async (req, res) => {
    try{

        var {rent,owner_principal,path,timeperiod,storage_size} = req.body;
        var principal = Principal.fromText(owner_principal);
        var result = await accountActor.create_storage(BigInt(rent),principal,path,timeperiod,BigInt(storage_size));
        result = result[0];
        console.log(result);
        if(result != null){

            var storage_principal = Principal.fromUint8Array(result.Id._arr).toText();
            result["Id"] = storage_principal;
            
            var owener_principal = Principal.fromUint8Array(result.OwnerPrincipal._arr);
            result["OwnerPrincipal"] = owener_principal.toText();
            result = JSON.parse(JSON.stringify(result, (key, value) =>
                typeof value === 'bigint'
                    ? value.toString()
                    : value // return everything else unchanged
            ));
            console.log(result);
            res.send(result);
        }else{
            res.send(result);
        }
    }catch(err){
        console.log(err);
        res.send(err);
    }
            
});


router.post('/add_rentee', async (req, res) => {
    try{
        const { storage_principal, rentee_principal,duration } = req.body;
        const storage_principal_ = Principal.fromText(storage_principal);
        const rentee_principal_ = Principal.fromText(rentee_principal);
        var result = await accountActor.add_rentee(storage_principal_,rentee_principal_,duration);
        result = result[0];
        var renter_principal = Principal.fromUint8Array(result._arr).toText();

        res.send({renter_principal:renter_principal,status:"success"});

    }catch(err){
        console.log(err);
        res.send(err);
    }
});

router.post('/remove_rentee', async (req, res) => {
    try{
        const { storage_principal, rentee_principal } = req.body;
        const storage_principal_ = Principal.fromText(storage_principal);
        const rentee_principal_ = Principal.fromText(rentee_principal);
        var result = await accountActor.remove_rentee(storage_principal_,rentee_principal_);
        result = result[0];
        console.log(result);
        res.send({status:"success",result:result});

    }catch(err){
        console.log(err);
        res.send(err);
    }
});

router.get('/get_available_storages', async (req, res) => {
    try{
        var result = await accountActor.get_available_storages();
        if(result != null){
        result = result.map((storage) => {
            const principal = Principal.fromUint8Array(storage.Id._arr);   
            storage["Id"] =principal.toText();
            var owener_principal = Principal.fromUint8Array(storage.OwnerPrincipal._arr);
            storage["OwnerPrincipal"] = owener_principal.toText();

            storage = JSON.parse(JSON.stringify(storage, (key, value) =>
            typeof value === 'bigint'
                ? value.toString()
                : value
            ));
            return storage;
        });
        res.send(result);}
        else{
            res.send([]);
        }
    }catch(err){
        console.log(err);
    }
});


// router.post('/delete_account', async (req, res) => {
//     try{

//     }catch (err){
//         console.log(err);
//         res.send(err);
//     }
// });

router.get('/get_balance', async (req, res) => {
    try{
        const { account_principal } = req.body;
        const principal = Principal.fromText(account_principal);
        const result = await accountActor.get_balance(principal);
        console.log(result);
        res.send({balance:result.toString()});
    }catch(err){
        console.log(err);
        res.send(err);
    }
});


router.post('/delete_storage', async (req, res) => {
    try{
        const { storage_principal } = req.body;
        const principal = Principal.fromText(storage_principal);
        const result = await accountActor.delete_storage(principal);
        console.log(result);
        res.send({status:"success"});

    }catch (err){
        console.log(err);
        res.send(err);
    }
});

export default router;