
import cors from 'cors';
import  express from 'express';
import BodyParser from 'body-parser';
import accountRoute from './Routes/account.js';

const app = express();



app.use(cors());
app.use(express.json());
app.use(BodyParser.json());
app.use(BodyParser.urlencoded({ extended: true }));

app.use('/api/account',accountRoute );

const port = process.env.PORT || 8080;

app.listen(port, () => {
    console.log(`Listening on port ${port}`);
});
