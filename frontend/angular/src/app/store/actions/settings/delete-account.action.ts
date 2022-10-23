import {createAction, props} from "@ngrx/store";

export enum DeleteAccountActionTypes {
  REQUEST_ACCOUNT_DELETE = '[ACTION] Request Account Deletion',
  REQUEST_ACCOUNT_DELETE_RESPONSE_OK = '[ACTION] Request Account Deletion Response OK',
  REQUEST_ACCOUNT_DELETE_RESPONSE_ERROR = '[ACTION] Request Account Deletion Response Error',
}

export const RequestDeleteAccountAction = createAction(
  DeleteAccountActionTypes.REQUEST_ACCOUNT_DELETE,
  props<{payload: { } }>()
);

export const RequestDeleteAccountResponseOkAction = createAction(
  DeleteAccountActionTypes.REQUEST_ACCOUNT_DELETE_RESPONSE_OK,
  props<{payload: { } }>()
);

export const RequestDeleteAccountResponseErrorAction = createAction(
  DeleteAccountActionTypes.REQUEST_ACCOUNT_DELETE_RESPONSE_ERROR,
  props<{payload: { } }>()
);

