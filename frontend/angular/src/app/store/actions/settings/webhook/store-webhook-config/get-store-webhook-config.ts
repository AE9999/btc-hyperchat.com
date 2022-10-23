import {createAction, props} from "@ngrx/store";
import {IStoreWebhookConfig} from "../../../../../model/settings/store-webhook-config/store-webhook-config";

export enum GetStoreWebhookConfigActionTypes {
  REQUEST_GET_STORE_WEBHOOK_CONFIG = '[ACTION] Request Get Store Webhook Config',
  REQUEST_GET_STORE_WEBHOOK_CONFIG_OK = '[ACTION] Request Get Store Webhook Config Ok',
  REQUEST_GET_STORE_WEBHOOK_CONFIG_ERROR = '[ACTION] Request Get Store Webhook Config Error',
}

export const RequestGetStoreWebhookConfigAction = createAction(
  GetStoreWebhookConfigActionTypes.REQUEST_GET_STORE_WEBHOOK_CONFIG,
  props<{payload: { } }>()
);

export const RequestGetStoreWebhookConfigOkAction = createAction(
  GetStoreWebhookConfigActionTypes.REQUEST_GET_STORE_WEBHOOK_CONFIG_OK,
  props<{payload: { storeWebhookConfig: IStoreWebhookConfig } }>()
);

export const RequestGetStoreWebhookConfigErrorAction = createAction(
  GetStoreWebhookConfigActionTypes.REQUEST_GET_STORE_WEBHOOK_CONFIG_ERROR,
  props<{payload: { } }>()
);
