import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface _SERVICE {
  'create_account' : ActorMethod<[string, string], [] | [Principal]>,
  'delete_account' : ActorMethod<[Principal], [] | [string]>,
  'get_account' : ActorMethod<[Principal], [] | [string]>,
  'get_all_accounts' : ActorMethod<[], Array<Principal>>,
  'get_balance' : ActorMethod<[Principal], [] | [bigint]>,
}
