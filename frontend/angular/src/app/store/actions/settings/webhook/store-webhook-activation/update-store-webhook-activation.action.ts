import {createAction, props} from "@ngrx/store";
import {IStoreWebhookActivation} from "../../../../../model/settings/store-webhook-activation.ts/store-webhook-activation";


export enum UpdateStoreWebhookActivationActionTypes {
  REQUEST_UPDATE_STORE_WEBHOOK_ACTIVATION = '[ACTION] Request Update Store Webhook Activation',
  REQUEST_UPDATE_STORE_WEBHOOK_ACTIVATION_OK = '[ACTION] Request Update Store Webhook Activation Ok',
  REQUEST_UPDATE_STORE_WEBHOOK_ACTIVATION_ERROR = '[ACTION] Request Update Store Webhook Activation Error',
}

export const RequestUpdateStoreWebhookActivationAction = createAction(
  UpdateStoreWebhookActivationActionTypes.REQUEST_UPDATE_STORE_WEBHOOK_ACTIVATION,
  props<{payload: { storeWebhookActivation: IStoreWebhookActivation } }>()
);

export const RequestUpdateStoreWebhookActivationOkAction = createAction(
  UpdateStoreWebhookActivationActionTypes.REQUEST_UPDATE_STORE_WEBHOOK_ACTIVATION_OK,
  props<{payload: { } }>()
);

export const RequestUpdateStoreWebhookActivationErrorAction = createAction(
  UpdateStoreWebhookActivationActionTypes.REQUEST_UPDATE_STORE_WEBHOOK_ACTIVATION_ERROR,
  props<{payload: { } }>()
);
