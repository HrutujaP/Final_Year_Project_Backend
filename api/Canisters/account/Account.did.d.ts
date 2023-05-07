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
export interface StorageStruct {
  'Id' : Principal,
  'Space' : bigint,
  'Name' : string,
  'UsedSpace' : bigint,
  'Path' : string,
  'Rent' : bigint,
  'Description' : string,
  'RenterPrincipal' : [] | [Principal],
  'RenteeDuration' : [] | [string],
  'Files' : Array<string>,
  'Timings' : string,
  'FileExt' : Array<string>,
  'TimePeriod' : string,
  'OwnerPrincipal' : Principal,
}
export interface Storages {
  'Rented_Storages' : Array<Principal>,
  'My_Storages' : Array<Principal>,
}
export interface _SERVICE {
  'add_balance' : ActorMethod<[Principal, bigint], [] | [bigint]>,
  'add_file' : ActorMethod<
    [Principal, string, string, bigint],
    [] | [StorageStruct]
  >,
  'add_rentee' : ActorMethod<[Principal, Principal, string], [] | [Principal]>,
  'create_account' : ActorMethod<[string, string], [] | [Principal]>,
  'create_storage' : ActorMethod<
    [bigint, Principal, string, bigint, Array<string>],
    [] | [StorageStruct]
  >,
  'delete_account' : ActorMethod<[Principal], [] | [string]>,
  'delete_storage' : ActorMethod<[Principal], [] | [string]>,
  'get_account' : ActorMethod<[Principal], [] | [Account]>,
  'get_all_accounts' : ActorMethod<[], Array<Principal>>,
  'get_all_storages' : ActorMethod<[Principal], [] | [Storages]>,
  'get_available_storages' : ActorMethod<[], Array<StorageStruct>>,
  'get_balance' : ActorMethod<[Principal], bigint>,
  'get_storage' : ActorMethod<[Principal], [] | [StorageStruct]>,
  'remove_file' : ActorMethod<
    [Principal, string, string, bigint],
    [] | [StorageStruct]
  >,
  'remove_rentee' : ActorMethod<[Principal, Principal], [] | [string]>,
  'withdraw_balance' : ActorMethod<[Principal, bigint], [] | [bigint]>,
}
