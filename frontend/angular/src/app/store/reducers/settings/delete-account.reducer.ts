import {Action, createReducer, on } from '@ngrx/store';
import {
  RequestDeleteAccountAction,
  RequestDeleteAccountResponseOkAction,
  RequestDeleteAccountResponseErrorAction

} from "../../actions/settings/delete-account.action";

export interface IDeleteAccountState {
  loading: boolean,
  error: boolean,
}

const initialState: IDeleteAccountState = {
  loading: false,
  error: false,
}

const reducer = createReducer(
  initialState,
  on (RequestDeleteAccountAction, (state, action) => {
    return {
      ...state,
      loading: true,
      error: false,
    }
  }),
  on (RequestDeleteAccountResponseOkAction, (state, action) => {
    return {
      loading: false,
      error: false,
    }
  }),
  on (RequestDeleteAccountResponseErrorAction, (state, action) => {
    return {
      loading: false,
      error: true,
    }
  }),
)

export function deleteAccountReducer(state: IDeleteAccountState | undefined, action: Action) {
  return reducer(state, action);
}
