
import cors from 'cors';
import  express from 'express';
import { Actor,HttpAgent } from '@dfinity/agent';
import {Principal } from '@dfinity/principal';

const app = express();


app.use(cors());
app.use(express.json());

import { idlFactory } from './account/Account.did.js';

const port = process.env.PORT || 3000;
app.listen(port, () => {
    console.log(`Listening on port ${port}`);
    });

    
app.post('/api/account', async (req, res) => {
        
        async function CreateAccount(name, email) {
            const agent = new HttpAgent({ host: 'http://localhost:8005' });
            const accountActor =await Actor.createActor(idlFactory, { agent, canisterId: Principal.fromText('ryjl3-tyaaa-aaaaa-aaaba-cai') });
            const result =await  accountActor.create_account(String(name), String(email));
            console.log(result);
         }
  
        var name = req.body.name;
        var email = req.body.email;

        await CreateAccount(name, email);
      
   
    }
);
