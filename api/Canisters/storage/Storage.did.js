export const idlFactory = ({ IDL }) => {
  const StorageStruct = IDL.Record({
    'Id' : IDL.Principal,
    'Space' : IDL.Int,
    'Path' : IDL.Text,
    'Rent' : IDL.Int,
    'RenterPrincipal' : IDL.Opt(IDL.Principal),
    'RenteeDuration' : IDL.Opt(IDL.Text),
    'TimePeriod' : IDL.Text,
    'OwnerPrincipal' : IDL.Principal,
  });
  return IDL.Service({
    'addRentee' : IDL.Func(
        [IDL.Principal, IDL.Principal, IDL.Text],
        [IDL.Opt(IDL.Principal)],
        [],
      ),
    'deleteAdvertisement' : IDL.Func([IDL.Principal], [IDL.Opt(IDL.Text)], []),
    'getAdvertisement' : IDL.Func(
        [IDL.Principal],
        [IDL.Opt(StorageStruct)],
        ['query'],
      ),
    'postAdvertisement' : IDL.Func(
        [IDL.Int, IDL.Principal, IDL.Text, IDL.Text, IDL.Int],
        [IDL.Opt(StorageStruct)],
        [],
      ),
    'removeRentee' : IDL.Func([IDL.Principal], [IDL.Opt(IDL.Text)], []),
  });
};
export const init = ({ IDL }) => { return []; };
