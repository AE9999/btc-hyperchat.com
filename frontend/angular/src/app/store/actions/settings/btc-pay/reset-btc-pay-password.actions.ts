import {createAction, props} from "@ngrx/store";

export enum BtcPayResetPasswordActionTypes {
  REQUEST_BTC_PAY_PASSWORD_RESET = '[ACTION] Request BtcPay Password Reset',
  REQUEST_BTC_PAY_PASSWORD_RESET_RESPONSE_OK = '[ACTION] Request BtcPay Password Reset Response OK',
  REQUEST_BTC_PAY_PASSWORD_RESET_RESPONSE_ERROR = '[ACTION] Request BtcPay Password Reset Error',
}

export const RequestBtcPayPasswordResetAction = createAction(
  BtcPayResetPasswordActionTypes.REQUEST_BTC_PAY_PASSWORD_RESET,
  props<{payload: { } }>()
);

export const RequestBtcPayPasswordResetResponseOkAction = createAction(
  BtcPayResetPasswordActionTypes.REQUEST_BTC_PAY_PASSWORD_RESET_RESPONSE_OK,
  props<{payload: { } }>()
);

export const RequestBtcPayPasswordResetResponseErrorAction = createAction(
  BtcPayResetPasswordActionTypes.REQUEST_BTC_PAY_PASSWORD_RESET_RESPONSE_ERROR,
  props<{payload: { } }>()
);

