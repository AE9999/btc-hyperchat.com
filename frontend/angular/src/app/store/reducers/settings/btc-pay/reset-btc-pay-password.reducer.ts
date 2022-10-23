import {Action, createReducer, on } from '@ngrx/store';
import {
  RequestBtcPayPasswordResetAction,
  RequestBtcPayPasswordResetResponseOkAction,
  RequestBtcPayPasswordResetResponseErrorAction

} from "../../../actions/settings/btc-pay/reset-btc-pay-password.actions";

export interface IResetBtcPayPasswordState {
  loading: boolean,
  error: boolean,
}

const initialState: IResetBtcPayPasswordState = {
  loading: false,
  error: false,
}

const reducer = createReducer(
  initialState,
  on (RequestBtcPayPasswordResetAction, (state, action) => {
    return {
      ...state,
      loading: true,
      error: false,
    }
  }),
  on (RequestBtcPayPasswordResetResponseOkAction, (state, action) => {
    return {
      loading: false,
      error: false,
    }
  }),
  on (RequestBtcPayPasswordResetResponseErrorAction, (state, action) => {
    return {
      loading: false,
      error: true,
    }
  }),
)

export function resetBtcPayPasswordReducer(state: IResetBtcPayPasswordState | undefined, action: Action) {
  return reducer(state, action);
}
