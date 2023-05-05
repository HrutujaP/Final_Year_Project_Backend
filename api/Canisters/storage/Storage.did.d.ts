import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface StorageStruct {
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
  'addRentee' : ActorMethod<[Principal, Principal, string], [] | [Principal]>,
  'deleteAdvertisement' : ActorMethod<[Principal], [] | [string]>,
  'getAdvertisement' : ActorMethod<[Principal], [] | [StorageStruct]>,
  'postAdvertisement' : ActorMethod<
    [bigint, Principal, string, string, bigint],
    [] | [StorageStruct]
  >,
  'removeRentee' : ActorMethod<[Principal], [] | [string]>,
}
