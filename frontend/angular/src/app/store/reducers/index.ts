import { ActionReducerMap } from "@ngrx/store";
import { IRegisterBtcChatState, registerBtcChatReducer } from "./btc-chat/register-btc-chat/register-btc-chat.reducer";
import { IGetMyReceivedBtcChatsState,getMyReceivedBtcChatsReducer } from "./btc-chat/my-received-btc-chats/get-my-received-btc-chats.reducer";
import {IGetProfileState, getProfileReducer} from "./profile/get-profile.reducer";
import {ITopProfilesState, topProfilesReducer} from "./profile/top-profiles.reducer";
import {IFindProfilesState, findProfilesReducer} from "./profile/find-profiles.reducer";
import {ITestWebhookState, testWebhookReducer} from "./settings/webhook/test-webhook.reducer";
import {getTestBtcChatReducer, IGetTestBtcChatState} from "./settings/webhook/get-test-btc-chat.reducer";
import {
  getStoreWebhookActivationReducer,
  IGetStoreWebhookActivationState
} from "./settings/webhook/store-webhook-activation/get-store-webhook-activation.reducer";
import {
  IUpdateStoreWebhookActivationState, updateStoreWebhookActivationReducer
} from "./settings/webhook/store-webhook-activation/update-store-webhook-activation.reducer";
import {
  getStoreWebhookConfigReducer,
  IGetStoreWebhookConfigState
} from "./settings/webhook/store-webhook-config/get-store-webhook-config.reducer";
import {
  IUpdateStoreWebhookConfigState,
  updateStoreWebhookConfigReducer
} from "./settings/webhook/store-webhook-config/update-store-webhook-config.reducer";
import {getBtcPayConfigReducer, IGetBtcPayConfigState} from "./settings/btc-pay/btc-pay-config.reducer";
import {IResetBtcPayPasswordState, resetBtcPayPasswordReducer} from "./settings/btc-pay/reset-btc-pay-password.reducer";
import {deleteAccountReducer, IDeleteAccountState} from "./settings/delete-account.reducer";
import {IBtcPayInvoiceModalState, btcPayInvoiceModalReducer} from "./btc-chat/register-btc-chat/btc-pay-invoice-modal.reducer";
import {IProcessBtcChatState, processBtcChatReducer} from "./btc-chat/my-received-btc-chats/process-btc-chat.reducer";

export interface AppState {
  // btc-chat
    // my-received-btc-chats
    getMyReceivedBtcChatsState: IGetMyReceivedBtcChatsState,
    processBtcChatState: IProcessBtcChatState,

    //register-btc-chat
    registerBtcChatState: IRegisterBtcChatState,
    btcPayInvoiceModalState: IBtcPayInvoiceModalState,

  // profile
  findProfilesState : IFindProfilesState ,
  getProfileState : IGetProfileState,
  topProfilesState: ITopProfilesState,

  // settings
    // btc-pay
    getBtcPayConfigState: IGetBtcPayConfigState,
    resetBtcPayPasswordState: IResetBtcPayPasswordState,
    // webhook
      // store-webhook-activation
      getStoreWebhookActivationState: IGetStoreWebhookActivationState,
      updateStoreWebhookActivationState: IUpdateStoreWebhookActivationState,
      // store-webhook-config
      getStoreWebhookConfigState: IGetStoreWebhookConfigState,
      updateStoreWebhookConfigState: IUpdateStoreWebhookConfigState,
    getTestBtcChatState: IGetTestBtcChatState,
    testWebhookState: ITestWebhookState,
  deleteAccountState: IDeleteAccountState,
}
//
export const reducers: ActionReducerMap<AppState> = {
  // btc-chat
  // my-received-btc-chats
  getMyReceivedBtcChatsState: getMyReceivedBtcChatsReducer,
  processBtcChatState: processBtcChatReducer,

  //register-btc-chat
  registerBtcChatState: registerBtcChatReducer,
  btcPayInvoiceModalState: btcPayInvoiceModalReducer,

  // profile
  findProfilesState : findProfilesReducer ,
  getProfileState : getProfileReducer,
  topProfilesState: topProfilesReducer,

  // settings
    // btc-pay
      getBtcPayConfigState: getBtcPayConfigReducer,
      resetBtcPayPasswordState: resetBtcPayPasswordReducer,
    // webhook
    // store-webhook-activation
      getStoreWebhookActivationState: getStoreWebhookActivationReducer,
      updateStoreWebhookActivationState: updateStoreWebhookActivationReducer,
    // store-webhook-config
      getStoreWebhookConfigState: getStoreWebhookConfigReducer,
      updateStoreWebhookConfigState: updateStoreWebhookConfigReducer,
    getTestBtcChatState: getTestBtcChatReducer,
    testWebhookState: testWebhookReducer,
  deleteAccountState: deleteAccountReducer,
};

// btc-chat
  // my-received-btc-chats
  export const selectGetMyReceivedBtcChatsState = (state: AppState) => state.getMyReceivedBtcChatsState;
  export const selectProcessBtcChatState = (state: AppState) => state.processBtcChatState;

  //register-btc-chat
  export const selectRegisterBtcChatState = (state: AppState) => state.registerBtcChatState;
  export const selectBtcPayInvoiceModalState = (state: AppState) => state.btcPayInvoiceModalState;

// profile
export const selectGetProfileState = (state: AppState) => state.getProfileState;
export const selectTopProfilesState = (state: AppState) => state.topProfilesState;
export const selectFindProfilesState = (state: AppState) => state.findProfilesState;

// settings
  // btc-pay
  export const selectGetBtcPayConfigState = (state: AppState) => state.getBtcPayConfigState;
  export const selectResetBtcPayPasswordState = (state: AppState) => state.resetBtcPayPasswordState;
  // webhook
    // store-webhook-activation
    export const selectGetStoreWebhookActivationState = (state: AppState) => state.getStoreWebhookActivationState;
    export const selectUpdateStoreWebhookActivationState = (state: AppState) => state.updateStoreWebhookActivationState;
    // store-webhook-config
    export const selectGetStoreWebhookConfigState = (state: AppState) => state.getStoreWebhookConfigState;
    export const selectUpdateStoreWebhookConfigState = (state: AppState) => state.updateStoreWebhookConfigState;
  export const selectGetTestBtcChatState = (state: AppState) => state.getTestBtcChatState;
  export const selectTestWebhookState = (state: AppState) => state.testWebhookState;
export const selectDeleteAccountState = (state: AppState) => state.deleteAccountState;
