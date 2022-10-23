import {createAction, props} from "@ngrx/store";
import {IStoreWebhookConfig} from "../../../../../model/settings/store-webhook-config/store-webhook-config";


export enum UpdateStoreWebhookConfigActionTypes {
  REQUEST_UPDATE_STORE_WEBHOOK_CONFIG = '[ACTION] Request Update Store Webhook Config',
  REQUEST_UPDATE_STORE_WEBHOOK_CONFIG_OK = '[ACTION] Request Update Store Webhook Config Ok',
  REQUEST_UPDATE_STORE_WEBHOOK_CONFIG_ERROR = '[ACTION] Request Update Store Webhook Config Error',
}

export const RequestUpdateStoreWebhookConfigAction = createAction(
  UpdateStoreWebhookConfigActionTypes.REQUEST_UPDATE_STORE_WEBHOOK_CONFIG,
  props<{payload: { storeWebhookConfig: IStoreWebhookConfig } }>()
);


export const RequestUpdateStoreWebhookConfigOkAction = createAction(
  UpdateStoreWebhookConfigActionTypes.REQUEST_UPDATE_STORE_WEBHOOK_CONFIG_OK,
  props<{payload: { storeWebhookConfig: IStoreWebhookConfig } }>()
);

export const RequestUpdateStoreWebhookConfigErrorAction = createAction(
  UpdateStoreWebhookConfigActionTypes.REQUEST_UPDATE_STORE_WEBHOOK_CONFIG_ERROR,
  props<{payload: { } }>()
);
