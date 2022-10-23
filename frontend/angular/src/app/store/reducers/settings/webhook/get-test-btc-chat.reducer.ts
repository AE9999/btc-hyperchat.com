import {Action, createReducer, on} from '@ngrx/store';
import {
  RequestGetTestBtcChatAction,
  RequestGetTestBtcChatOkAction,
  RequestGetTestBtcChatErrorAction
} from "../../../actions/settings/webhook/get-test-btc-chat.action";

import {IMyReceivedBtcChat} from "../../../../model/btc-chat/my-received-btc-chats/my-received-btc-chat";


export interface IGetTestBtcChatState {
  loading: boolean,
  error: boolean,
  myReceivedBtcChat: IMyReceivedBtcChat | undefined,
}

const initialState: IGetTestBtcChatState = {
  loading: false,
  error: false,
  myReceivedBtcChat: undefined,
}

const reducer = createReducer(
  initialState,
  on (RequestGetTestBtcChatAction, (state, action) => {
    return {
      ...state,
      loading: true,
      error: false,
    }
  }),
  on (RequestGetTestBtcChatOkAction, (state, action) => {
    return {
      ...state,
      loading: false,
      error: false,
      myReceivedBtcChat: action.payload.myReceivedBtcChat
    }
  }),
  on (RequestGetTestBtcChatErrorAction, (state, action) => {
    return {
      ...state,
      error: false,
      loading : false,
    }
  }),
)

export function getTestBtcChatReducer(state: IGetTestBtcChatState | undefined, action: Action) {
  return reducer(state, action);
}
