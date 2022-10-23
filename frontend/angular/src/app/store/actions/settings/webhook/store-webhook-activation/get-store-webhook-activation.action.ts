import {createAction, props} from "@ngrx/store";
import {IStoreWebhookActivation} from "../../../../../model/settings/store-webhook-activation.ts/store-webhook-activation";


export enum GetStoreWebhookActivationActionTypes {
  REQUEST_GET_STORE_WEBHOOK_ACTIVATION = '[ACTION] Request Get Store Webhook Activation',
  REQUEST_GET_STORE_WEBHOOK_ACTIVATION_OK = '[ACTION] Request Get Store Webhook Activation Ok',
  REQUEST_GET_STORE_WEBHOOK_ACTIVATION_ERROR = '[ACTION] Request Get Store Webhook Activation Error',
}

export const RequestGetStoreWebhookActivationAction = createAction(
  GetStoreWebhookActivationActionTypes.REQUEST_GET_STORE_WEBHOOK_ACTIVATION,
  props<{payload: { } }>()
);

export const RequestGetStoreWebhookActivationOkAction = createAction(
  GetStoreWebhookActivationActionTypes.REQUEST_GET_STORE_WEBHOOK_ACTIVATION_OK,
  props<{payload: { storeWebhookActivation: IStoreWebhookActivation } }>()
);

export const RequestGetStoreWebhookActivationErrorAction = createAction(
  GetStoreWebhookActivationActionTypes.REQUEST_GET_STORE_WEBHOOK_ACTIVATION_ERROR,
  props<{payload: { } }>()
);
