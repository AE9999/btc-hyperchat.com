import {createAction, props} from "@ngrx/store";
import {IMyReceivedBtcChat} from "../../../../model/btc-chat/my-received-btc-chats/my-received-btc-chat";

export enum DeliverNewMyReceivedBtcChatActionTypes {
  DELIVER_NEW_MY_RECEIVED_BTC_CHAT = '[ACTION] Deliver new My Received BtcChat'
}
export const DeliverNewMyReceivedBtcChatAction = createAction(
  DeliverNewMyReceivedBtcChatActionTypes.DELIVER_NEW_MY_RECEIVED_BTC_CHAT,
  props<{payload: { myReceivedBtcChat: IMyReceivedBtcChat } }>()
);
