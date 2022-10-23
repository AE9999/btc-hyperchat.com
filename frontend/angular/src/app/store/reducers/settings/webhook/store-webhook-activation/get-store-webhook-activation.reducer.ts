import {Action, createReducer, on } from '@ngrx/store';
import {
  RequestGetStoreWebhookActivationAction,
  RequestGetStoreWebhookActivationOkAction,
  RequestGetStoreWebhookActivationErrorAction

} from "../../../../actions/settings/webhook/store-webhook-activation/get-store-webhook-activation.action";
import {IStoreWebhookActivation} from "../../../../../model/settings/store-webhook-activation.ts/store-webhook-activation";
import {
  RequestUpdateStoreWebhookConfigOkAction
} from "../../../../actions/settings/webhook/store-webhook-config/update-store-webhook-config";


export interface IGetStoreWebhookActivationState {
  loading: boolean,
  error: boolean,
  storeWebhookActivation : IStoreWebhookActivation | undefined,
}

const initialState: IGetStoreWebhookActivationState = {
  loading: false,
  error: false,
  storeWebhookActivation: undefined,
}

const reducer = createReducer(
  initialState,
  on (RequestGetStoreWebhookActivationAction, (state, action) => {
    return {
      ...state,
      loading: true,
      error: false,
    }
  }),
  on (RequestGetStoreWebhookActivationOkAction, (state, action) => {
    return {
      ...state,
      loading: false,
      error: false,
      storeWebhookActivation: action.payload.storeWebhookActivation,
    }
  }),
  on (RequestGetStoreWebhookActivationErrorAction, (state, action) => {
    return {
      ...state,
      loading: false,
      error: true,
      storeWebhookActivation: undefined,
    }
  }),
  on (RequestUpdateStoreWebhookConfigOkAction, (state, action) => {


    if (!state.storeWebhookActivation // nothing to update
       || (action.payload.storeWebhookConfig?.url
           && action.payload.storeWebhookConfig?.postTypeCode) // Valid configuration
       ) {
      return {
        ...state // We don't neet to do anything
      }
    }

    const storeWebhookActivation : IStoreWebhookActivation =
      JSON.parse(JSON.stringify(state.storeWebhookActivation));

    // The backend should have disable this, we do this now automatically.
    storeWebhookActivation.webhookActive = false; //
    storeWebhookActivation.automaticallyProcessBtcChatsIfWebhookSucceeds = false;

    return {
      ...state,
      storeWebhookActivation: storeWebhookActivation,
    }
  }),
)

export function getStoreWebhookActivationReducer(state: IGetStoreWebhookActivationState | undefined, action: Action) {
  return reducer(state, action);
}
