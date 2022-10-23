import {
  RequestProcessBtcChatAction,
  RequestProcessBtcChatResponseOkAction,
  RequestProcessBtcChatResponseErrorAction,
} from "../../../actions/btc-chat/my-received-btc-chats/process-btc-chat.action"
import {Action, createReducer, on} from "@ngrx/store";

export interface IProcessBtcChatState {
  error: boolean,
  loading: boolean,
  idsOfMyReceivedBtcChatBeingMarked: string[],
}

const initialState: IProcessBtcChatState = {
  error: false,
  loading: false,
  idsOfMyReceivedBtcChatBeingMarked: [],
}

const reducer = createReducer(
  initialState,
  on(RequestProcessBtcChatAction, (state, action) => {
    let idsOfMyReceivedBtcChatBeingMarked : string[] =
      JSON.parse(JSON.stringify(state.idsOfMyReceivedBtcChatBeingMarked));

    idsOfMyReceivedBtcChatBeingMarked.push(action.payload.processBtcChatRequest.btcChatId)

    return {
      ...state,
      idsOfMyReceivedBtcChatBeingMarked: idsOfMyReceivedBtcChatBeingMarked
    }
  }),
  on(RequestProcessBtcChatResponseOkAction, (state, action) => {
    let idToRemove = action.payload.processBtcChatResponse.btcChatId;

    let idsOfMyReceivedBtcChatBeingMarked : string[] =
      JSON.parse(JSON.stringify(state.idsOfMyReceivedBtcChatBeingMarked));

    const index = idsOfMyReceivedBtcChatBeingMarked.indexOf(idToRemove);
    if (index > -1) {
      // only splice array when item is found
      idsOfMyReceivedBtcChatBeingMarked.splice(index, 1); // 2nd parameter means remove one item only
    }
    return {
      ...state,
      idsOfMyReceivedBtcChatBeingMarked: idsOfMyReceivedBtcChatBeingMarked,
    }

  }),
  on(RequestProcessBtcChatResponseErrorAction, (state, action) => {
    let idToRemove = action.payload.processBtcChatRequest.btcChatId;

    let idsOfMyReceivedBtcChatBeingMarked : string[] =
      JSON.parse(JSON.stringify(state.idsOfMyReceivedBtcChatBeingMarked));

    const index = idsOfMyReceivedBtcChatBeingMarked.indexOf(idToRemove);
    if (index > -1) { // only splice array when item is found
      idsOfMyReceivedBtcChatBeingMarked.splice(index, 1); // 2nd parameter means remove one item only
    }

    return {...state, idsOfMyReceivedBtcChatBeingMarked: idsOfMyReceivedBtcChatBeingMarked }
  }),
);


export function processBtcChatReducer(state: IProcessBtcChatState | undefined, action: Action) {
  return reducer(state, action);
}
