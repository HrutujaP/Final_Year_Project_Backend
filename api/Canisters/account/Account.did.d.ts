import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface Account {
  'Id' : Principal,
  'Email' : string,
  'Rented_Storages' : Array<Principal>,
  'Name' : string,
  'My_Storages' : Array<Principal>,
  'Balance' : bigint,
}
export interface Storage {
  'Id' : Principal,
  'Space' : bigint,
  'Path' : string,
  'Rent' : bigint,
  'RenterPrincipal' : [] | [Principal],
  'RenteeDuration' : [] | [string],
  'TimePeriod' : string,
  'OwnerPrincipal' : Principal,
}
export interface _SERVICE {
  'add_balance' : ActorMethod<[Principal, bigint], bigint>,
  'add_rentee' : ActorMethod<[Principal, Principal, string], [] | [Principal]>,
  'create_account' : ActorMethod<[string, string], [] | [Principal]>,
  'create_storage' : ActorMethod<
    [bigint, Principal, string, string, bigint],
    [] | [Storage]
  >,
  'delete_account' : ActorMethod<[Principal], [] | [string]>,
  'delete_storage' : ActorMethod<[Principal], [] | [string]>,
  'get_account' : ActorMethod<[Principal], [] | [Account]>,
  'get_all_accounts' : ActorMethod<[], Array<Principal>>,
  'get_balance' : ActorMethod<[Principal], bigint>,
  'get_storage' : ActorMethod<[Principal], [] | [Storage]>,
  'remove_rentee' : ActorMethod<[Principal, Principal], [] | [string]>,
  'withdraw_balance' : ActorMethod<[Principal, bigint], bigint>,
}
