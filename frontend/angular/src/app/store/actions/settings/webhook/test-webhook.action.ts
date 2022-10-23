import {createAction, props} from "@ngrx/store";

export enum TestWebhookActionTypes {
  REQUEST_WEBHOOK_TEST = '[ACTION] Request Webhook test',
  REQUEST_WEBHOOK_TEST_RESPONSE_OK = '[ACTION] Request Webhook Test Response OK',
  REQUEST_WEBHOOK_TEST_RESPONSE_FAILURE = '[ACTION] Request Webhook Test Response Failure',
  REQUEST_WEBHOOK_TEST_RESPONSE_ERROR = '[ACTION] Request Webhook Test Response Error',
}

export const RequestWebhookTestAction = createAction(
  TestWebhookActionTypes.REQUEST_WEBHOOK_TEST,
  props<{payload: { } }>()
);

export const RequestWebhookTestResponseOkAction = createAction(
  TestWebhookActionTypes.REQUEST_WEBHOOK_TEST_RESPONSE_OK,
  props<{payload: { } }>()
);

export const RequestWebhookTestResponseFailureAction = createAction(
  TestWebhookActionTypes.REQUEST_WEBHOOK_TEST_RESPONSE_FAILURE,
  props<{payload: { } }>()
);

export const RequestWebhookResponseErrorAction = createAction(
  TestWebhookActionTypes.REQUEST_WEBHOOK_TEST_RESPONSE_ERROR,
  props<{payload: { } }>()
);
