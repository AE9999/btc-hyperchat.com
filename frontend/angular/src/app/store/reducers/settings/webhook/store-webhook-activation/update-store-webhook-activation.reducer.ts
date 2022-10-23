import {Action, createReducer, on } from '@ngrx/store';
import {
  RequestUpdateStoreWebhookActivationAction,
  RequestUpdateStoreWebhookActivationOkAction,
  RequestUpdateStoreWebhookActivationErrorAction

} from "../../../../actions/settings/webhook/store-webhook-activation/update-store-webhook-activation.action";


export interface IUpdateStoreWebhookActivationState {
  loading: boolean,
  error: boolean,
}

const initialState: IUpdateStoreWebhookActivationState = {
  loading: false,
  error: false,
}

const reducer = createReducer(
  initialState,
  on (RequestUpdateStoreWebhookActivationAction, (state, action) => {
    return {
      ...state,
      loading: true,
      error: false,
    }
  }),
  on (RequestUpdateStoreWebhookActivationOkAction, (state, action) => {
    return {
      ...state,
      loading: false,
      error: false,
    }
  }),
  on (RequestUpdateStoreWebhookActivationErrorAction, (state, action) => {
    return {
      ...state,
      loading: false,
      error: true,
    }
  }),
)

export function updateStoreWebhookActivationReducer(state: IUpdateStoreWebhookActivationState | undefined, action: Action) {
  return reducer(state, action);
}
