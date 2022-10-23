import {Action, createAction, createReducer, on, props} from '@ngrx/store';
import {
  RequestWebhookTestAction,
  RequestWebhookResponseErrorAction,
  RequestWebhookTestResponseFailureAction,
  RequestWebhookTestResponseOkAction

} from "../../../actions/settings/webhook/test-webhook.action";

export interface ITestWebhookState {
  loading: boolean,
  ok: boolean,
  failure: boolean,
  error: boolean,
}

const initialState: ITestWebhookState = {
  loading: false,
  ok: false,
  failure: false,
  error: false,
}

const reducer = createReducer(
  initialState,
  on(RequestWebhookTestAction, (state, action) => {
    return {
      ...state,
      loading: true,
      ok : false,
      failure: false,
      error: false,
    }
  }),
  on (RequestWebhookTestResponseOkAction, (state, action) => {
    return {
      ...state,
      loading: false,
      ok : true,
      failure: false,
      error: false,
    }
  }),
  on (RequestWebhookTestResponseFailureAction, (state, action) => {
    return {
      ...state,
      loading : false,
      updating: false,
      failure: true,
      error: false,
    }
  }),
  on (RequestWebhookResponseErrorAction, (state, action) => {
    return {
      ...state,
      loading : false,
      updating: false,
      failure: false,
      error: true,
    }
  })
)

export function testWebhookReducer(state: ITestWebhookState | undefined, action: Action) {
  return reducer(state, action);
}
