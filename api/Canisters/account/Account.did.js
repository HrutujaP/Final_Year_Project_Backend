export const idlFactory = ({ IDL }) => {
  const Storage = IDL.Record({
    'Id' : IDL.Principal,
    'Space' : IDL.Int,
    'Path' : IDL.Text,
    'Rent' : IDL.Int,
    'RenterPrincipal' : IDL.Opt(IDL.Principal),
    'RenteeDuration' : IDL.Opt(IDL.Text),
    'TimePeriod' : IDL.Text,
    'OwnerPrincipal' : IDL.Principal,
  });
  const Account = IDL.Record({
    'Id' : IDL.Principal,
    'Email' : IDL.Text,
    'Rented_Storages' : IDL.Vec(IDL.Principal),
    'Name' : IDL.Text,
    'My_Storages' : IDL.Vec(IDL.Principal),
    'Balance' : IDL.Int,
  });
  const Storages = IDL.Record({
    'Rented_Storages' : IDL.Vec(IDL.Principal),
    'My_Storages' : IDL.Vec(IDL.Principal),
  });
  return IDL.Service({
    'add_balance' : IDL.Func([IDL.Principal, IDL.Nat], [IDL.Opt(IDL.Nat)], []),
    'add_rentee' : IDL.Func(
        [IDL.Principal, IDL.Principal, IDL.Text],
        [IDL.Opt(IDL.Principal)],
        [],
      ),
    'create_account' : IDL.Func(
        [IDL.Text, IDL.Text],
        [IDL.Opt(IDL.Principal)],
        [],
      ),
    'create_storage' : IDL.Func(
        [IDL.Int, IDL.Principal, IDL.Text, IDL.Text, IDL.Int],
        [IDL.Opt(Storage)],
        [],
      ),
    'delete_account' : IDL.Func([IDL.Principal], [IDL.Opt(IDL.Text)], []),
    'delete_storage' : IDL.Func([IDL.Principal], [IDL.Opt(IDL.Text)], []),
    'get_account' : IDL.Func([IDL.Principal], [IDL.Opt(Account)], ['query']),
    'get_all_accounts' : IDL.Func([], [IDL.Vec(IDL.Principal)], ['query']),
    'get_all_storages' : IDL.Func(
        [IDL.Principal],
        [IDL.Opt(Storages)],
        ['query'],
      ),
    'get_balance' : IDL.Func([IDL.Principal], [IDL.Nat], ['query']),
    'get_storage' : IDL.Func([IDL.Principal], [IDL.Opt(Storage)], ['query']),
    'remove_rentee' : IDL.Func(
        [IDL.Principal, IDL.Principal],
        [IDL.Opt(IDL.Text)],
        [],
      ),
    'withdraw_balance' : IDL.Func(
        [IDL.Principal, IDL.Nat],
        [IDL.Opt(IDL.Nat)],
        [],
      ),
  });
};
export const init = ({ IDL }) => { return []; };
