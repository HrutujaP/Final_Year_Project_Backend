import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

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
  'addRentee' : ActorMethod<[Principal, Principal, string], [] | [Principal]>,
  'deleteAdvertisement' : ActorMethod<[Principal], [] | [string]>,
  'getAdvertisement' : ActorMethod<[Principal], [] | [Storage]>,
  'postAdvertisement' : ActorMethod<
    [bigint, Principal, string, string, bigint],
    [] | [Storage]
  >,
  'removeRentee' : ActorMethod<[Principal], [] | [string]>,
}
