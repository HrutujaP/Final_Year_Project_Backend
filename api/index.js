
import cors from 'cors';
import  express from 'express';
import { Actor,HttpAgent } from '@dfinity/agent';
import {Principal } from '@dfinity/principal';
import { idlFactory } from './account/Account.did.js';


const app = express();


app.use(cors());
app.use(express.json());


const port = process.env.PORT || 3000;
app.listen(port, () => {
    console.log(`Listening on port ${port}`);
    });


app.post('/api/account', async (req, res) => {
        
        async function CreateAccount(name, email) {
            const agent = new HttpAgent({ host: 'http://127.0.0.1:8000' });
            // log(agent);
            const accountActor =await Actor.createActor(idlFactory, { agent, canisterId: Principal.fromText('rrkah-fqaaa-aaaaa-aaaaq-cai') });
            const result =await  accountActor.create_account(String(name), String(email));
            console.log(result);
            return
         }
  
        var name = req.body.name;
        var email = req.body.email;

        await CreateAccount(name, email);
        res.send('Account created');
   
    }
);
