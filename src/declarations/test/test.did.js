export const idlFactory = ({ IDL }) => {
  return IDL.Service({ 'test' : IDL.Func([], [], []) });
};
export const init = ({ IDL }) => { return []; };
