export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'create_account' : IDL.Func([IDL.Text, IDL.Text, IDL.Int], [IDL.Text], []),
    'get_account' : IDL.Func([IDL.Int], [IDL.Text], ['query']),
    'get_all_accounts' : IDL.Func([], [IDL.Text], ['query']),
    'get_balance' : IDL.Func([IDL.Int], [IDL.Int], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };
