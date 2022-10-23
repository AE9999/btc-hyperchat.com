import {Action, createReducer, on } from '@ngrx/store';
import {
  RequestGetStoreWebhookConfigAction,
  RequestGetStoreWebhookConfigOkAction,
  RequestGetStoreWebhookConfigErrorAction
} from "../../../../actions/settings/webhook/store-webhook-config/get-store-webhook-config";
import {IStoreWebhookConfig} from "../../../../../model/settings/store-webhook-config/store-webhook-config";
import {
  RequestUpdateStoreWebhookConfigOkAction
} from "../../../../actions/settings/webhook/store-webhook-config/update-store-webhook-config";


export interface IGetStoreWebhookConfigState {
  loading: boolean,
  error: boolean,
  storeWebhookConfig : IStoreWebhookConfig | undefined,
}

const initialState: IGetStoreWebhookConfigState = {
  loading: false,
  error: false,
  storeWebhookConfig: undefined,
}

const reducer = createReducer(
  initialState,
  on (RequestGetStoreWebhookConfigAction, (state, action) => {
    return {
      ...state,
      loading: true,
      error: false,
    }
  }),
  on (RequestGetStoreWebhookConfigOkAction, (state, action) => {
    return {
      ...state,
      loading: false,
      error: false,
      storeWebhookConfig: action.payload.storeWebhookConfig,
    }
  }),
  on (RequestGetStoreWebhookConfigErrorAction, (state, action) => {
    return {
      ...state,
      loading: false,
      error: true,
      storeWebhookConfig: undefined,
    }
  }),
  on (RequestUpdateStoreWebhookConfigOkAction, (state, action) => {
    return {
      ...state,
      loading: false,
      error: false,
      storeWebhookConfig: action.payload.storeWebhookConfig,
    }
  }),

)

export function getStoreWebhookConfigReducer(state: IGetStoreWebhookConfigState | undefined, action: Action) {
  return reducer(state, action);
}
