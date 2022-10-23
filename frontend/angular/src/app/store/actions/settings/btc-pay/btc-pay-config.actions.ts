import {createAction, props} from "@ngrx/store";
import {IBtcPayConfig} from "../../../../model/settings/btc-pay/btc-pay-config";

export enum BtcPayConfigActionTypes {
  REQUEST_BTC_PAY_CONFIG = '[ACTION] Request BtcPay Config',
  REQUEST_BTC_PAY_CONFIG_RESPONSE_OK = '[ACTION] Request BtcPay Config Response OK',
  REQUEST_BTC_PAY_CONFIG_RESPONSE_ERROR = '[ACTION] Request BtcPay Config Response Error',
}

export const RequestBtcPayConfigAction = createAction(
  BtcPayConfigActionTypes.REQUEST_BTC_PAY_CONFIG,
  props<{payload: { } }>()
);

export const RequestBtcPayConfigResponseOkAction = createAction(
  BtcPayConfigActionTypes.REQUEST_BTC_PAY_CONFIG_RESPONSE_OK,
  props<{payload: {  btcPayConfig: IBtcPayConfig } }>()
);

export const RequestBtcPayConfigResponseErrorAction = createAction(
  BtcPayConfigActionTypes.REQUEST_BTC_PAY_CONFIG_RESPONSE_ERROR,
  props<{payload: {} }>()
);

