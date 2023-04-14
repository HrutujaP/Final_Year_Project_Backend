export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'create_account' : IDL.Func(
        [IDL.Text, IDL.Text],
        [IDL.Opt(IDL.Principal)],
        [],
      ),
    'delete_account' : IDL.Func([IDL.Principal], [IDL.Opt(IDL.Text)], []),
    'get_account' : IDL.Func([IDL.Principal], [IDL.Opt(IDL.Text)], ['query']),
    'get_all_accounts' : IDL.Func([], [IDL.Vec(IDL.Principal)], ['query']),
    'get_balance' : IDL.Func([IDL.Principal], [IDL.Opt(IDL.Int)], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };
