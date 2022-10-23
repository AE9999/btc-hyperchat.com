import {Action, createReducer, on} from '@ngrx/store';
import {
  RegisterBtcChatRequestAction,
  RegisterBtcChatResponseOkAction,
  RegisterBtcChatResponseErrorAction,
} from "../../../actions/btc-chat/register-btc-chat/register-btc-chat.actions"

export interface IRegisterBtcChatState {
  error: boolean,
  sending: boolean,
}

const initialState: IRegisterBtcChatState = {
  error: false,
  sending: false,
}

const reducer = createReducer(
  initialState,
  on (RegisterBtcChatRequestAction, (state, action) => {
    return {...state, sending: true, error: false, showing_invoice: false }
  }),
  on (RegisterBtcChatResponseOkAction, (state, action) => {
    return {...state, sending: false, error: false, showing_invoice: false }
  }),
  on (RegisterBtcChatResponseErrorAction, (state, action) => {
    return {...state, sending: false, error: true, showing_invoice: false }
  }),
);

export function registerBtcChatReducer(state: IRegisterBtcChatState | undefined, action: Action) {
  return reducer(state, action);
}
