import express from 'express';
import { Actor, } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import { idlFactory } from '../Canisters/account/Account.did.js';
import { agent } from '../agent.js';
import admin from 'firebase-admin'
import Multer from 'multer';

const serviceAccount = {
    "type": "service_account",
    "project_id": "disk-space-renting",
    "private_key_id": "7dd4cb7ddc22325f91ed211dd0c95061da5b619e",
    "private_key": "-----BEGIN PRIVATE KEY-----\nMIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQClKWFMY3Qhgay6\nZnhwwt6V4hNLWlCjKheLmg82q+jLcdhv+dHW9/5N1uBGw+Hg91i213oQIMFVHlW4\nAj1sCPWu2jL2QwdxAjiwb5yXy+O1FVvsN1qTtC6jsTI3w29zPGMOCfafkrBaRLMJ\n6NKLkPfcbSydAMXyp3O3cRJLoziBLKDiO8yIjr4ICP6fWyknKZGhNGN5ffNGi/9W\nlDPq9WdBccug5Vubag+y1OpJy01eiXVliTDOCGX0o9SYkxXF345tcriNqzUVZMTq\nkBRkCuT5GdEeAJ1V4ADerfS/N9tXN9Xks4WWeT2QrDG4DVuiIerL4VredszLQK+H\n0fQlb1q9AgMBAAECggEAI5L6Jh310ZHAxpVzs7YFg3sidMSLQdm7IJKaXEhC6C8p\nU1UX4Zmg6Nszi4p9iJs+PIPFAtxLSC40gGHbIP39DSBqgaiGXc0AsOAZdoiwDzz+\ng66Yj9/re86a5m/t/YAwRcHXoR9TZN4PQj8nunALMytAxMB54n426ngsXKZXcVr2\n5PzLgb/Y9Sg3oeoJOIBw+LOrKvA8fKjorn/UwqQXoZMkx5Dns6C4tl9UX6/hUdNh\nujG8I8sZdFYK08KYY+bpk+E4D+aCgggKP78y7Oo7GC/bwW9L2tcgXNFIT6Gjl83Q\n9ecvI+E9V0q3z3E5LTgYJTSMbvO5/yHRuTZONNqmgQKBgQDQ3FMy7dCfPlW+gp6T\nhmyvd3llBJorcdodOeOmAnpr6RWs50E6VGtp1ew/VcP2oxhednbMqn2HLW69eMMZ\nwPUBFesnbLs/hMQuacYnVnV+LydmCOQufN8nD+Lr69MXt1y6Iry5YaBPiYruuZGV\nyaz8kGbWMrqbVp1CGzLOUAX5FQKBgQDKcDAeBQcxfEbb0iIon8O/ijMNkg/4DgzT\nirns9zDgHJuNKDygSdDwvhPs6VxEWSKI8OUnwZ/1DwhrdSsYHsD1dhdrGLGHlO8A\nILeY1Hjkdb6ycK0O1gwuiaGFJVSZ9ZMiEusG6OOk0tODXzxJnBQ1J5bsxwIIoqIb\nLfHts5d1CQKBgQCgeuoik8Bc21Se0VZvHDaejDpmeCbNgumI01Yr1HOkPrA87mOc\nIgl2hLzF3Wx5MdAjpPqbP9S77RjN1OB0BWM7tk+9NzqZD7Z9wyBSUYhtsGWhbOuH\nHMOKDBbcF5NXAbjp/I7uEilPc+ugik1mvlOArKOXqdrgSKkEhkl/ZNVyEQKBgBV/\ntJx7jDgFKsFNn7MQxIki9CkMygVd+J6+rOAtoFMstphXHZkoPbhTd2pBnjrj/Hbu\nnmeKeKJd5lcDKoB1Q6jN0c7d4Hy1pGwvDVB9wBkDKC1rKfXZZZHLrAun20Q+gFGu\nEKe7M/HdMflRs+VdXggHHFApEl44Z17bkkXcPMmBAoGAacQ2tEAh2DqOHq1mzQpY\ntxy4aJBwIu3cHnQ6BTVrNFJfsOwMqf7HC+E5H8WMUbMuRg4xXQ2xRv/1W54J5SN/\nc7YDk7H6miEwTdwuNhkwNr8tyCLxVetxJt5Gpk8nbctkxSuiHWBwLHNJooEtr3Re\n+BOCkQERvg1qOv8B8BtlwaE=\n-----END PRIVATE KEY-----\n",
    "client_email": "firebase-adminsdk-uxlpj@disk-space-renting.iam.gserviceaccount.com",
    "client_id": "113771774431327851197",
    "auth_uri": "https://accounts.google.com/o/oauth2/auth",
    "token_uri": "https://oauth2.googleapis.com/token",
    "auth_provider_x509_cert_url": "https://www.googleapis.com/oauth2/v1/certs",
    "client_x509_cert_url": "https://www.googleapis.com/robot/v1/metadata/x509/firebase-adminsdk-uxlpj%40disk-space-renting.iam.gserviceaccount.com",
    "universe_domain": "googleapis.com"
}

admin.initializeApp({
    credential: admin.credential.cert(serviceAccount),
    storageBucket: 'disk-space-renting.appspot.com'
})
var router = express.Router();

// const accountActor = Actor.createActor(idlFactory, { agent, canisterId: Principal.fromText('rrkah-fqaaa-aaaaa-aaaaq-cai') });
const accountActor = Actor.createActor(idlFactory, { agent, canisterId: Principal.fromText('yvrmz-eaaaa-aaaao-aifqq-cai') });
const upload = Multer({ storage: Multer.memoryStorage() });


router.post('/create_account', async (req, res) => {

    try {

        const { name, email } = req.body;
        const result = await accountActor.create_account(String(name), String(email));
        // console.log(result);
        res.send(Principal.fromUint8Array(result[0]._arr).toText());
    }
    catch (err) {
        console.log(err);
        res.send(err);
    }
});

router.get('/get_accounts', async (req, res) => {
    try {

        const result = await accountActor.get_all_accounts();
        const accounts = await Promise.all(
            result.map(async (account) => {
                const principal = Principal.fromUint8Array(account._arr);
                const str_principal = principal.toText();
                var holder_name = await accountActor.get_account(principal);
                holder_name = holder_name[0].Name;
                return { Principal: str_principal, holder_name: holder_name };
            })
        );
        res.send(accounts);
    }
    catch (err) {
        console.log(err);
        res.send(err);
    }
});


router.get('/get_account', async (req, res) => {
    try {
        const account_principal = req.query.account_principal.toString();
        const principal = Principal.fromText(account_principal);
        var result = await accountActor.get_account(principal);
        result = result[0];

        if (result == null) {
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

    } catch (err) {
        console.log(err);
        res.send(err);
    }

});

router.get('/get_storages', async (req, res) => {
    try {
        const account_principal = req.query.account_principal.toString();
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
    try {
        const { account_principal, amount } = req.body;
        const principal = Principal.fromText(account_principal);
        const result = await accountActor.add_balance(principal, BigInt(amount));
        console.log(result);
        res.send({ updated_balance: result.toString() });

    } catch (err) {
        console.log(err);
        res.send(err);
    }
});


router.post('/withdraw_balance', async (req, res) => {
    try {
        const { account_principal, amount } = req.body;
        const principal = Principal.fromText(account_principal);
        var result = await accountActor.withdraw_balance(principal, BigInt(amount));
        result = result[0];
        // console.log(result);
        if (result != null) {

            res.send({ updated_balance: result.toString() });
        } else {
            res.send({ updated_balance: "Account not found" });
        }

    } catch (err) {
        console.log(err);
        res.send(err);
    }
});



router.get('/get_storage', async (req, res) => {
    try {
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
        if (renter != null) {
            var renter_principal = Principal.fromUint8Array(renter._arr);
            result["RenterPrincipal"] = renter_principal.toText();
        }

        result = JSON.parse(JSON.stringify(result, (key, value) =>
            typeof value === 'bigint'
                ? value.toString()
                : value // return everything else unchanged
        ));

        res.send(result);
    } catch (err) {
        console.log(err);
        res.send(err);
    }
});



router.post('/create_storage', async (req, res) => {
    try {

        var { rent, owner_principal, path, timeperiod, storage_size, name, description, timings } = req.body;
        var principal = Principal.fromText(owner_principal);
        var result = await accountActor.create_storage(BigInt(rent), principal, timeperiod, BigInt(storage_size), [name, description, timings, path]);
        result = result[0];
        console.log(result);
        if (result != null) {

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
        } else {
            res.send(result);
        }
    } catch (err) {
        console.log(err);
        res.send(err);
    }

});
router.post('/add_file', upload.array('files'), async (req, res) => {
    try {
        const storage_principal = req.query.id.toString();
        const files = req.files;

        if (!files || files.length === 0) {
            res.status(400).send('No files were uploaded.');
            return;
        }

        const bucket = admin.storage().bucket();
        const uploadPromises = [];
        const firestore = admin.firestore();
        const fileUrlsCollection = firestore.collection(storage_principal);

        for (const file of files) {
            const fileName = Date.now() + '_' + file.originalname;
            console.log('fileName:', fileName);

            const fileUpload = bucket.file(fileName);

            if (!fileUpload) {
                console.error('Error creating fileUpload object.');
                res.status(500).send('Error uploading files.');
                return;
            }

            const metadata = {
                metadata: {
                    contentType: file.mimetype,
                },
            };

            const uploadPromise = fileUpload.save(file.buffer, metadata);
            uploadPromises.push(uploadPromise);

            const downloadUrl = await fileUpload.getSignedUrl({
                action: 'read',
                expires: '03-01-2500', // Adjust the expiration date as needed
            });

            const documentRef = fileUrlsCollection.doc(file.originalname);
            await documentRef.set({ downloadUrl: downloadUrl[0] });
            console.log('Document added:', documentRef.id);

            const principal = Principal.fromText(storage_principal);
            const result = await accountActor.add_file(principal, file.originalname, file.originalname.split('.').pop(), BigInt(file.size));
            console.log(result);

        }

        await Promise.all(uploadPromises);

        // Send the response once all operations are completed
        res.send({ status: 'success' });
    } catch (error) {
        console.error('Error uploading files:', error);
        res.status(500).send('Error uploading files.');
    }
});

router.post('/delete_file', async (req, res) => {
    try {
      const collectionId = req.query.collectionId;
      const docId = req.query.docId;
      const size = req.query.size;
      const fileext = docId.split('.').pop();
      console.log(docId,size,collectionId,fileext);
  
      const firestore = admin.firestore();
      const fileUrlsCollection = firestore.collection(collectionId);
      const documentRef = fileUrlsCollection.doc(docId);
  
      const docSnapshot = await documentRef.get();
      if (!docSnapshot.exists) {
        res.status(404).send('Document not found.');
        return;
      }
  
      await documentRef.delete();
      const principal = Principal.fromText(collectionId);
      const result = await accountActor.remove_file(principal,docId,fileext,BigInt(size));
console.log(result);

      res.send({ status: 'success' });
    } catch (error) {
      console.error('Error deleting file:', error);
      res.status(500).send('Error deleting file.');
    }
  });
  



router.post('/add_rentee', async (req, res) => {
    try {
        const { storage_principal, rentee_principal, duration } = req.body;
        const storage_principal_ = Principal.fromText(storage_principal);
        const rentee_principal_ = Principal.fromText(rentee_principal);
        var result = await accountActor.add_rentee(storage_principal_, rentee_principal_, duration);
        result = result[0];
        var renter_principal = Principal.fromUint8Array(result._arr).toText();

        res.send({ renter_principal: renter_principal, status: "success" });

    } catch (err) {
        console.log(err);
        res.send(err);
    }
});

router.post('/remove_rentee', async (req, res) => {
    try {
        const { storage_principal, rentee_principal } = req.body;
        const storage_principal_ = Principal.fromText(storage_principal);
        const rentee_principal_ = Principal.fromText(rentee_principal);
        var result = await accountActor.remove_rentee(storage_principal_, rentee_principal_);
        result = result[0];
        console.log(result);
        res.send({ status: "success", result: result });

    } catch (err) {
        console.log(err);
        res.send(err);
    }
});

router.get('/get_available_storages', async (req, res) => {
    try {
        var result = await accountActor.get_available_storages();
        if (result != null) {
            result = result.map((storage) => {
                const principal = Principal.fromUint8Array(storage.Id._arr);
                storage["Id"] = principal.toText();
                var owener_principal = Principal.fromUint8Array(storage.OwnerPrincipal._arr);
                storage["OwnerPrincipal"] = owener_principal.toText();

                storage = JSON.parse(JSON.stringify(storage, (key, value) =>
                    typeof value === 'bigint'
                        ? value.toString()
                        : value
                ));
                return storage;
            });
            res.send(result);
        }
        else {
            res.send([]);
        }
    } catch (err) {
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
    try {
        const { account_principal } = req.body;
        const principal = Principal.fromText(account_principal);
        const result = await accountActor.get_balance(principal);
        console.log(result);
        res.send({ balance: result.toString() });
    } catch (err) {
        console.log(err);
        res.send(err);
    }
});


router.post('/delete_storage', async (req, res) => {
    try {
        const { storage_principal } = req.body;
        const principal = Principal.fromText(storage_principal);
        const result = await accountActor.delete_storage(principal);
        console.log(result);
        res.send({ status: "success" });

    } catch (err) {
        console.log(err);
        res.send(err);
    }
});

export default router;