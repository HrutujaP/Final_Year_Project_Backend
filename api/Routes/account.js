import express from 'express';
import { Actor,} from '@dfinity/agent';
import {Principal } from '@dfinity/principal';
import { idlFactory } from '../Canisters/account/Account.did.js';
import { agent } from '../agent.js';


var router = express.Router();

const accountActor = Actor.createActor(idlFactory, { agent, canisterId: Principal.fromText('rrkah-fqaaa-aaaaa-aaaaq-cai') });
// const accountActor = Actor.createActor(idlFactory, { agent, canisterId: Principal.fromText('yvrmz-eaaaa-aaaao-aifqq-cai') });

/* This code defines a route for creating a new account using the POST method. It expects the request
body to contain a JSON object with `name` and `email` properties. It then calls the `create_account`
method of the `accountActor` actor with the provided name and email, and sends the resulting
principal (converted from a Uint8Array) back as a response. If an error occurs, it logs the error
and sends it back as a response. */
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


/* This code defines a route for getting all the accounts using the GET method. It calls the
`get_all_accounts` method of the `accountActor` actor to get a list of all the accounts. It then
maps over the list of accounts and converts each account's principal from a Uint8Array to a string
using the `Principal.fromUint8Array` and `principal.toText` methods. It also calls the `get_account`
method of the `accountActor` actor to get the holder name for each account. Finally, it sends back
an array of objects containing the principal and holder name for each account as a response. If an
error occurs, it logs the error and sends it back as a response. */
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


/* This code defines a route for getting the details of a specific account using the GET method. It
expects the request body to contain a JSON object with an `account_principal` property, which is the
principal of the account whose details are being requested. It then calls the `get_account` method
of the `accountActor` actor with the provided principal to get the account details. The account
details include the holder name, the list of storages rented by the account, and the list of
storages owned by the account. The code then maps over the lists of rented and owned storages to
convert each storage's principal from a Uint8Array to a string using the `Principal.fromUint8Array`
and `principal.toText` methods. Finally, it sends back an object containing the account details as a
response. If an error occurs, it logs the error and sends it back as a response. */
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

/* This code defines a route for getting the details of all the storages rented and owned by a specific
account using the GET method. It expects the request body to contain a JSON object with an
`account_principal` property, which is the principal of the account whose storages are being
requested. It then calls the `get_all_storages` method of the `accountActor` actor with the provided
principal to get the list of storages rented and owned by the account. The code then maps over the
lists of rented and owned storages to convert each storage's principal from a Uint8Array to a string
using the `Principal.fromUint8Array` and `principal.toText` methods. Finally, it sends back an
object containing the list of rented and owned storages as a response. If an error occurs, it logs
the error and sends it back as a response. */
router.get('/get_storages', async (req, res) => {
    try {
        const { account_principal } = req.body; 
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


/* This code defines a route for adding balance to a specific account using the POST method. It expects
the request body to contain a JSON object with an `account_principal` property, which is the
principal of the account to which balance is being added, and an `amount` property, which is the
amount of balance being added. It then calls the `add_balance` method of the `accountActor` actor
with the provided principal and amount to add the balance to the account. The updated balance is
then sent back as a response in a JSON object with a `updated_balance` property. If an error occurs,
it logs the error and sends it back as a response. */
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

/* This code defines a route for withdrawing balance from a specific account using the POST method. It
expects the request body to contain a JSON object with an `account_principal` property, which is the
principal of the account from which balance is being withdrawn, and an `amount` property, which is
the amount of balance being withdrawn. It then calls the `withdraw_balance` method of the
`accountActor` actor with the provided principal and amount to withdraw the balance from the
account. The updated balance is then sent back as a response in a JSON object with a
`updated_balance` property. If an error occurs, it logs the error and sends it back as a response. */
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


/* This code defines a route for getting the details of a specific storage using the GET method. It
expects the request body to contain a JSON object with a `storage_principal` property, which is the
principal of the storage whose details are being requested. It then calls the `get_storage` method
of the `accountActor` actor with the provided principal to get the storage details. The storage
details include the owner principal, the storage size, the storage price, and the storage status.
The code then converts the owner principal from a Uint8Array to a string using the
`Principal.fromUint8Array` and `principal.toText` methods. Finally, it sends back an object
containing the storage details as a response. If an error occurs, it logs the error and sends it
back as a response. */
router.get('/get_storage',async (req, res) => {
    try{
        const storage_principal = req.query.storage_principal.toString();
        const principal = Principal.fromText(storage_principal);
        var result = await accountActor.get_storage(principal);
        result = result[0];

        result["Id"] = storage_principal
        var owener_principal = Principal.fromUint8Array(result.OwnerPrincipal._arr);
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


/* This code defines a route for creating a new storage using the POST method. It expects the request
body to contain a JSON object with `rent`, `owner_principal`, `path`, `timeperiod`, and
`storage_size` properties. It then calls the `create_storage` method of the `accountActor` actor
with the provided parameters to create a new storage. The resulting storage principal is then
converted from a Uint8Array to a string using the `Principal.fromUint8Array` and `principal.toText`
methods and added to the response object. The owner principal is also converted from a Uint8Array to
a string and added to the response object. Finally, the response object is sent back as a response. */
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

/* The above code is defining a route for a POST request to add a rentee to an account. It expects the
request body to contain the storage principal, rentee principal, and duration. It then converts the
storage and rentee principals from text to Principal objects, calls the add_rentee function on the
accountActor with these parameters, and retrieves the resulting renter principal. Finally, it sends
a response containing the renter principal and a status of "success". If an error occurs, it logs
the error and sends it as the response. */
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