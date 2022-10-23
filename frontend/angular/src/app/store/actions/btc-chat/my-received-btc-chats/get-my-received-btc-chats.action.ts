import {createAction, props} from "@ngrx/store";
import {
  IMyReceivedBtcChatsResponse
} from "../../../../model/btc-chat/my-received-btc-chats/my-received-btc-chats-response";

export enum GetMyReceivedBtcChatsActionTypes {
  REQUEST_GET_MY_RECEIVED_BTC_CHATS = '[ACTION] Request Get My Received BtcChats.',
  REQUEST_GET_MY_RECEIVED_BTC_CHATS_RESPONSE_OK = '[ACTION] Reques Get My Received BtcChats Response Ok.',
  REQUEST_GET_MY_RECEIVED_BTC_CHATS_RESPONSE_ERROR = '[ACTION] Request Get My Received BtcChats Response Error.',
}

export const RequestGetMyReceivedBtcChatsAction = createAction(
  GetMyReceivedBtcChatsActionTypes.REQUEST_GET_MY_RECEIVED_BTC_CHATS,
  props<{payload: { } }>()
);

export const RequestGetMyReceivedBtcChatsResponseOkAction = createAction(
  GetMyReceivedBtcChatsActionTypes.REQUEST_GET_MY_RECEIVED_BTC_CHATS_RESPONSE_OK,
  props<{payload: { myReceivedBtcChatsResponse: IMyReceivedBtcChatsResponse } }>()
);

export const RequestGetMyReceivedBtcChatsResponseErrorAction = createAction(
  GetMyReceivedBtcChatsActionTypes.REQUEST_GET_MY_RECEIVED_BTC_CHATS_RESPONSE_ERROR,
  props<{payload: {  } }>()
);
