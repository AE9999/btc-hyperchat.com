import { createAction, props } from '@ngrx/store';
import {IRegisterBtcChatRequest} from "../../../../model/btc-chat/register-btc-chat/register-btc-chat-request";
import {IRegisterBtcChatResponse} from "../../../../model/btc-chat/register-btc-chat/register-btc-chat-response";

export enum RegisterBtcChatActionTypes {
  REGISTER_BTC_CHAT_REQUEST = '[ACTION] Register BTC Chat Request.',
  REGISTER_BTC_CHAT_RESPONSE_OK = '[ACTION] Register BTC Chat response OK.',
  REGISTER_BTC_CHAT_RESPONSE_ERROR = '[ACTION] Register BTC Chat response Error.',
}

export const RegisterBtcChatRequestAction = createAction(
  RegisterBtcChatActionTypes.REGISTER_BTC_CHAT_REQUEST,
  props<{payload: { registerBtcChatRequest: IRegisterBtcChatRequest } }>()
);

export const RegisterBtcChatResponseOkAction = createAction(
  RegisterBtcChatActionTypes.REGISTER_BTC_CHAT_RESPONSE_OK,
  props<{payload: { registerBtcChatResponse: IRegisterBtcChatResponse } }>()
);

export const RegisterBtcChatResponseErrorAction = createAction(
  RegisterBtcChatActionTypes.REGISTER_BTC_CHAT_RESPONSE_ERROR,
  props<{payload: {  } }>()
);
