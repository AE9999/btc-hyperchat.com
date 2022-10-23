import {Action, createReducer, on } from '@ngrx/store';
import {
  RequestUpdateStoreWebhookConfigAction,
  RequestUpdateStoreWebhookConfigOkAction,
  RequestUpdateStoreWebhookConfigErrorAction

} from "../../../../actions/settings/webhook/store-webhook-config/update-store-webhook-config";


export interface IUpdateStoreWebhookConfigState {
  loading: boolean,
  error: boolean,
}

const initialState: IUpdateStoreWebhookConfigState = {
  loading: false,
  error: false,
}

const reducer = createReducer(
  initialState,
  on (RequestUpdateStoreWebhookConfigAction, (state, action) => {
    return {
      ...state,
      loading: true,
      error: false,
    }
  }),
  on (RequestUpdateStoreWebhookConfigOkAction, (state, action) => {
    return {
      ...state,
      loading: false,
      error: false,
    }
  }),
  on (RequestUpdateStoreWebhookConfigErrorAction, (state, action) => {
    return {
      ...state,
      loading: false,
      error: true,
    }
  }),
)

export function updateStoreWebhookConfigReducer(state: IUpdateStoreWebhookConfigState | undefined, action: Action) {
  return reducer(state, action);
}
