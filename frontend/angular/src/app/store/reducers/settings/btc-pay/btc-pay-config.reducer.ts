import {Action, createReducer, on } from '@ngrx/store';
import {
  RequestBtcPayConfigAction,
  RequestBtcPayConfigResponseOkAction,
  RequestBtcPayConfigResponseErrorAction

} from "../../../actions/settings/btc-pay/btc-pay-config.actions";
import {IBtcPayConfig} from "../../../../model/settings/btc-pay/btc-pay-config";


export interface IGetBtcPayConfigState {
  loading: boolean,
  error: boolean,
  btcPayConfig : IBtcPayConfig | undefined,
}

const initialState: IGetBtcPayConfigState = {
  loading: false,
  error: false,
  btcPayConfig: undefined,
}

const reducer = createReducer(
  initialState,
  on (RequestBtcPayConfigAction, (state, action) => {
    return {
      ...state,
      loading: true,
      error: false,
    }
  }),
  on (RequestBtcPayConfigResponseOkAction, (state, action) => {
    return {
      ...state,
      loading: false,
      error: false,
      btcPayConfig: action.payload.btcPayConfig,
    }
  }),
  on (RequestBtcPayConfigResponseErrorAction, (state, action) => {
    return {
      ...state,
      loading: false,
      error: true,
      btcPayConfig: undefined,
    }
  }),
)

export function getBtcPayConfigReducer(state: IGetBtcPayConfigState | undefined, action: Action) {
  return reducer(state, action);
}
