export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'create_account' : IDL.Func([IDL.Text, IDL.Text], [IDL.Text], []),
    'delete_account' : IDL.Func([IDL.Principal], [IDL.Text], []),
    'get_account' : IDL.Func([IDL.Principal], [IDL.Text], ['query']),
    'get_all_accounts' : IDL.Func([], [IDL.Vec(IDL.Int)], ['query']),
    'get_balance' : IDL.Func([IDL.Int], [IDL.Int], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };
