import { Injectable } from '@angular/core';
import {WindowService} from "./window.service";
import {select, Store} from "@ngrx/store";
import {AppState, selectBtcPayInvoiceModalState, selectTopProfilesState} from "../../store/reducers";
import {
  CloseModalCallbackReceivedAction, InvoicePayedCallbackReceivedAction,
  LoadModalCallbackReceivedAction
} from "../../store/actions/btc-chat/register-btc-chat/btc-pay-invoice-modal.action";
import {IBtcPayInvoiceModalState} from "../../store/reducers/btc-chat/register-btc-chat/btc-pay-invoice-modal.reducer";
import {combineLatest, Observable, withLatestFrom} from "rxjs";
import {first} from "rxjs/operators";

@Injectable({
  providedIn: 'root'
})
export class BtcPayInvoiceModalService {

  btcPayInvoiceModalState$: Observable<IBtcPayInvoiceModalState>;

  constructor(private windowService: WindowService,
              private store: Store<AppState>) {
    let me = this;

    let window = this.windowService.nativeWindow;
    window.btcpay.onModalReceiveMessage(this.handle_callback.bind(me))

    this.btcPayInvoiceModalState$ =
      store.pipe(select(selectBtcPayInvoiceModalState));
  }

  handle_callback(callback: any) {
    console.info("callback", callback);

    if (callback && callback.data) {
      console.info(`got a callback for`, callback.data);

      if (callback.data === "loaded") {
        this.store.dispatch(LoadModalCallbackReceivedAction({payload: {}}))
      } else if (callback.data === "close") {
        this.store.dispatch(CloseModalCallbackReceivedAction({payload: {}}))
      } else {
        let {invoiceId, status } = callback.data;
        console.info(`processing a callback for ${invoiceId}`, status);
        //withLatestFrom(this.btcPayInvoiceModalState$).pipe(first()).subscribe
        if (status === "received"
          || status == "confirmed"
          || status === "settled"
          || status === "processing") {
          combineLatest([this.btcPayInvoiceModalState$])
            .pipe(first())
            .subscribe(([btcPayInvoiceModalState]) => {
              if (btcPayInvoiceModalState.activeInvoices.has(invoiceId)) {
                const payload = {payload: {invoiceId: invoiceId}}
                this.store.dispatch(InvoicePayedCallbackReceivedAction(payload));
              }
            });
        }
      }
    }
  }

  display_invoice(invoiceId: string) {
    this.windowService.nativeWindow.btcpay.showInvoice(invoiceId);
  }




}
