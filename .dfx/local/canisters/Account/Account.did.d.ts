import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface _SERVICE {
  'create_account' : ActorMethod<[string, string], string>,
  'delete_account' : ActorMethod<[Principal], string>,
  'get_account' : ActorMethod<[Principal], string>,
  'get_all_accounts' : ActorMethod<[], Array<bigint>>,
  'get_balance' : ActorMethod<[bigint], bigint>,
}
