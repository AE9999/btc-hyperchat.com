import {
  CloseModalCallbackReceivedAction, InformUserAboutPayedInvoiceAction, InvoicePayedCallbackReceivedAction,
  LoadModalCallbackReceivedAction
} from "../../../actions/btc-chat/register-btc-chat/btc-pay-invoice-modal.action";
import {Action, createReducer, on} from "@ngrx/store";
import {RegisterBtcChatResponseOkAction} from "../../../actions/btc-chat/register-btc-chat/register-btc-chat.actions";

export interface IBtcPayInvoiceModalState {
  loading: boolean,
  hasPaymentToInformUserAbout: boolean,
  paymentMade: boolean,
  showingInvoice: boolean,
  activeInvoices: Set<string>,  // Should be more data in the future,
}

const initialState: IBtcPayInvoiceModalState = {
  loading: false,
  hasPaymentToInformUserAbout: false,
  paymentMade: false,
  showingInvoice: false,
  activeInvoices: new Set<string>()
}

const reducer = createReducer(
  initialState,
  on(RegisterBtcChatResponseOkAction, (state, action) => {

    // Per https://bobbyhadz.com/blog/javascript-convert-set-to-json
    let activeInvoices: Set<string> = new Set<string>(state.activeInvoices);

    activeInvoices.add(action.payload.registerBtcChatResponse.invoiceId)

    return { ...state,
             loading: true,
             activeInvoices: activeInvoices }
  }),
  on(LoadModalCallbackReceivedAction, (state, action) => {
    return {...state,
            loading: false,
            showingInvoice: true }
  }),
  on(CloseModalCallbackReceivedAction, (state, action) => {
    return {...state,
            loading: false,
            showingInvoice: false }
  }),
  on(InformUserAboutPayedInvoiceAction, (state, action) => {
    return { ...state,
            hasPaymentToInformUserAbout: false }
  }),
  on(InvoicePayedCallbackReceivedAction, (state, action) => {

    const invoiceId = action.payload.invoiceId;

    let activeInvoices: Set<string> = new Set<string>(state.activeInvoices);

    let hasPaymentToInformUserAbout = state.hasPaymentToInformUserAbout;

    if (activeInvoices.has(invoiceId)) {
      activeInvoices.delete(invoiceId);
      hasPaymentToInformUserAbout = true;
    }

    return { ...state,
             activeInvoices: activeInvoices,
             hasPaymentToInformUserAbout: hasPaymentToInformUserAbout }
  }),
);


export function btcPayInvoiceModalReducer(state: IBtcPayInvoiceModalState | undefined, action: Action) {
  return reducer(state, action);
}
