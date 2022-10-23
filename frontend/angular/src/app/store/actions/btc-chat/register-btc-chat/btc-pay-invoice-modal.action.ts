import {createAction, props} from "@ngrx/store";

export enum BtcPayInvoiceModalActionTypes {
  MODAL_DISPLAYED = '[ACTION] Modal Displayed.',
  LOAD_MODAL_CALLBACK_RECEIVED = '[ACTION] BtcPay Invoice Modal Load Modal Callback Received.',
  CLOSE_MODAL_CALLBACK_RECEIVED = '[ACTION] BtcPay Invoice Modal Close Modal Callback Received.',
  INVOICE_PAYED_CALLBACK_RECEIVED = '[ACTION] BtcPay Invoice Modal Invoice Payed Callback Received.',
  INVOICE_PAYED_CALLBACK_HANDLED = '[ACTION] BtcPay Invoice Modal Invoice Payed Callback Handled.',
  INFORM_USER_ABOUT_PAYED_INVOICE = '[ACTION] BtcPay Inform User About Payed Invoice.',
}

export const ModalDisplayedAction = createAction(
  BtcPayInvoiceModalActionTypes.MODAL_DISPLAYED,
  props<{payload: {  } }>()
);

export const LoadModalCallbackReceivedAction = createAction(
  BtcPayInvoiceModalActionTypes.LOAD_MODAL_CALLBACK_RECEIVED,
  props<{payload: {  } }>()
);

export const CloseModalCallbackReceivedAction = createAction(
  BtcPayInvoiceModalActionTypes.CLOSE_MODAL_CALLBACK_RECEIVED,
  props<{payload: {  } }>()
);

export const InvoicePayedCallbackReceivedAction = createAction(
  BtcPayInvoiceModalActionTypes.INVOICE_PAYED_CALLBACK_RECEIVED,
  props<{payload: { invoiceId: string } }>()
);

export const InvoicePayedCallbackHandledAction = createAction(
  BtcPayInvoiceModalActionTypes.INVOICE_PAYED_CALLBACK_HANDLED,
  props<{payload: { } }>()
);

export const InformUserAboutPayedInvoiceAction = createAction(
  BtcPayInvoiceModalActionTypes.INFORM_USER_ABOUT_PAYED_INVOICE,
  props<{payload: { } }>()
);


