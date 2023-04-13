import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface _SERVICE {
  'create_account' : ActorMethod<[string, string, bigint], string>,
  'get_account' : ActorMethod<[bigint], string>,
  'get_all_accounts' : ActorMethod<[], string>,
  'get_balance' : ActorMethod<[bigint], bigint>,
}
