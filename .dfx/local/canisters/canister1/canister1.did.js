export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'get_message' : IDL.Func([], [IDL.Text], ['query']),
    'set_message' : IDL.Func([IDL.Text], [IDL.Null], []),
  });
};
export const init = ({ IDL }) => { return []; };
