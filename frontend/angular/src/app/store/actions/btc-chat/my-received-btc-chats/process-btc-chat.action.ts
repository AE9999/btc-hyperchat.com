import {createAction, props} from "@ngrx/store";
import {IProcessBtcChatRequest} from "../../../../model/btc-chat/process-btc-chat/process-btc-chat-request";
import {IProcessBtcChatResponse} from "../../../../model/btc-chat/process-btc-chat/process-btc-chat-response";

export enum ProcessBtcChatActionTypes {
  REQUEST_PROCESS_BTC_CHAT = '[ACTION] Request Process Btc Chat.',
  REQUEST_PROCESS_BTC_CHAT_RESPONSE_OK = '[ACTION] Request Process Btc Chat Response Ok',
  REQUEST_PROCESS_BTC_CHAT_RESPONSE_ERROR = '[ACTION] Request Process Btc Chat Response Error',
}

export const RequestProcessBtcChatAction = createAction(
  ProcessBtcChatActionTypes.REQUEST_PROCESS_BTC_CHAT,
  props<{payload: { processBtcChatRequest: IProcessBtcChatRequest } }>()
);

export const RequestProcessBtcChatResponseOkAction = createAction(
  ProcessBtcChatActionTypes.REQUEST_PROCESS_BTC_CHAT_RESPONSE_OK,
  props<{payload: { processBtcChatResponse: IProcessBtcChatResponse } }>()
);

export const RequestProcessBtcChatResponseErrorAction = createAction(
  ProcessBtcChatActionTypes.REQUEST_PROCESS_BTC_CHAT_RESPONSE_ERROR,
  props<{payload: { processBtcChatRequest: IProcessBtcChatRequest } }>()
);



