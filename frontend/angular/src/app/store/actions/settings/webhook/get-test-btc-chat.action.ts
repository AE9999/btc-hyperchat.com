import {createAction, props} from "@ngrx/store";
import {IMyReceivedBtcChat} from "../../../../model/btc-chat/my-received-btc-chats/my-received-btc-chat";

export enum GetTestBtcChatActionTypes {
  REQUEST_GET_TEST_BTC_CHAT = '[ACTION] Request Get Test Btc Chat',
  REQUEST_GET_TEST_BTC_CHAT_OK = '[ACTION] Request Get Test Btc Chat Ok',
  REQUEST_GET_TEST_BTC_CHAT_ERROR = '[ACTION] Request Get Test Btc Chat Error',
}

export const RequestGetTestBtcChatAction = createAction(
  GetTestBtcChatActionTypes.REQUEST_GET_TEST_BTC_CHAT,
  props<{payload: { } }>()
);

export const RequestGetTestBtcChatOkAction = createAction(
  GetTestBtcChatActionTypes.REQUEST_GET_TEST_BTC_CHAT_OK,
  props<{payload: { myReceivedBtcChat: IMyReceivedBtcChat } }>()
);

export const RequestGetTestBtcChatErrorAction = createAction(
  GetTestBtcChatActionTypes.REQUEST_GET_TEST_BTC_CHAT_ERROR,
  props<{payload: { } }>()
);
