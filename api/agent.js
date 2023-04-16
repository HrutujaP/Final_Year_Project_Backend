import { identity } from './identity.js';
import { fetch,Headers } from 'cross-fetch';
import { HttpAgent } from '@dfinity/agent';

if(!globalThis.fetch) {
    globalThis.fetch = fetch;
    globalThis.Headers = fetch.Headers;
}

if(!global.fetch){
    global.fetch = fetch;
    global.Headers = fetch.Headers;
}

new Headers();

const agent = new HttpAgent({identity : identity, host: 'http://127.0.0.1:4943',fetch: fetch });
// const agent = new HttpAgent({identity : identity, host: 'https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.ic0.app/',fetch: fetch });
await agent.fetchRootKey();

export { agent };