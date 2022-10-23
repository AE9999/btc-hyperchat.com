import { Action, createReducer, on } from '@ngrx/store';

import {
  RequestGetMyReceivedBtcChatsAction,
  RequestGetMyReceivedBtcChatsResponseOkAction,
  RequestGetMyReceivedBtcChatsResponseErrorAction,
} from "../../../actions/btc-chat/my-received-btc-chats/get-my-received-btc-chats.action"

import {
  DeliverNewMyReceivedBtcChatAction
} from "../../../actions/btc-chat/my-received-btc-chats/deliver-new-my-received-btc-chat.action"

import {IMyReceivedBtcChat} from "../../../../model/btc-chat/my-received-btc-chats/my-received-btc-chat";
import {
  RequestProcessBtcChatResponseOkAction
} from "../../../actions/btc-chat/my-received-btc-chats/process-btc-chat.action";

export interface IGetMyReceivedBtcChatsState {
  error: boolean,
  loading: boolean,
  myReceivedBtcChats: IMyReceivedBtcChat[],
}

const initialState: IGetMyReceivedBtcChatsState = {
  error: false,
  loading: false,
  myReceivedBtcChats: [],
}

const reducer = createReducer(
  initialState,
  on (RequestGetMyReceivedBtcChatsAction, (state, action) => {
    return { ...state,
             loading: true,
             error: false }
  }),
  on (RequestGetMyReceivedBtcChatsResponseOkAction, (state, action) => {
    return { ...state,
             loading: false,
             error: false,
             myReceivedBtcChats: action.payload.myReceivedBtcChatsResponse.myReceivedBtcChats }
  }),
  on (RequestGetMyReceivedBtcChatsResponseErrorAction, (state, action) => {
    return { ...state,
             loading: false,
             error: true,
             myReceivedBtcChats: []
    }
  }),
  on(RequestProcessBtcChatResponseOkAction, (state, action) => {

    let myReceivedBtcChats: IMyReceivedBtcChat[] =
      JSON.parse(JSON.stringify(state.myReceivedBtcChats));

    let idToRemove = action.payload.processBtcChatResponse.btcChatId;

    const index = myReceivedBtcChats.findIndex(myReceivedBtcChat => myReceivedBtcChat.id === idToRemove);
    if (index > -1) { // only splice array when item is found
      myReceivedBtcChats.splice(index, 1); // 2nd parameter means remove one item only
    }

    return { ...state,
             myReceivedBtcChats: myReceivedBtcChats }

  }),
  on(DeliverNewMyReceivedBtcChatAction, (state, action) => {

    let myReceivedBtcChats: IMyReceivedBtcChat[] =
      JSON.parse(JSON.stringify(state.myReceivedBtcChats));

    myReceivedBtcChats.push(action.payload.myReceivedBtcChat);

    return {...state, myReceivedBtcChats: myReceivedBtcChats }
  }),
);

export function getMyReceivedBtcChatsReducer(state: IGetMyReceivedBtcChatsState | undefined, action: Action) {
  return reducer(state, action);
}
