import {Injectable} from "@angular/core";
import {Actions, createEffect, ofType} from "@ngrx/effects";
import {MatSnackBar} from "@angular/material/snack-bar";
import {TranslateService} from "@ngx-translate/core";
import {BtcPayInvoiceModalService} from "../../../../service/frontend/btc-pay-invoice-modal.service";
import {
  RegisterBtcChatResponseOkAction,
} from "../../../actions/btc-chat/register-btc-chat/register-btc-chat.actions";
import {catchError, combineLatest, map, Observable, of, switchMap} from "rxjs";
import {
  CloseModalCallbackReceivedAction, InformUserAboutPayedInvoiceAction,

  InvoicePayedCallbackHandledAction, InvoicePayedCallbackReceivedAction, ModalDisplayedAction
} from "../../../actions/btc-chat/register-btc-chat/btc-pay-invoice-modal.action";
import {select, Store} from "@ngrx/store";
import {AppState, selectBtcPayInvoiceModalState} from "../../../reducers";
import {first} from "rxjs/operators";
import {RegisterBtcChatFormResetService} from "../../../../service/frontend/register-btc-chat-form-reset.service";


@Injectable()
export class BtcPayInvoiceModalEffects {
  constructor(
    private readonly actions: Actions,
    private readonly snackBar: MatSnackBar,
    private store: Store<AppState>,
    private translate: TranslateService,
    private btcPayInvoiceModalService: BtcPayInvoiceModalService,
    private registerBtcChatFormResetService: RegisterBtcChatFormResetService,

  ) {}

  showInvoice$ = createEffect(() =>
    this.actions.pipe(
      ofType(RegisterBtcChatResponseOkAction),
      switchMap((action) => {

        const invoiceId = action.payload.registerBtcChatResponse.invoiceId

        this.translate.get('REGISTER_BTC_CHAT_EFFECT.GENERATING_INVOICE')
          .subscribe(message =>
            this.snackBar.open(message, "",{duration: 2000})
          );

        this.btcPayInvoiceModalService
          .display_invoice(invoiceId)

        return of(ModalDisplayedAction({payload: {}}))

      })
    )
  );

  handleInvoicePaid = createEffect(() =>
    this.actions.pipe(
      ofType(InvoicePayedCallbackReceivedAction),
      switchMap((action) => {

        this.translate.get('REGISTER_BTC_CHAT_EFFECT.INVOICE_PAYED')
          .subscribe(message =>
            this.snackBar.open(message, "",{duration: 2000})
          );

        return of(InvoicePayedCallbackHandledAction({payload: {}}))

      })
    )
  );

  //
  // https://stackoverflow.com/questions/61848083/how-can-i-clean-my-form-after-a-ngrx-succeeded-action
  // TODO(Clear stuff) in form.
  //

  showInvoicePaid = createEffect(() =>
    this.actions.pipe(
      ofType(CloseModalCallbackReceivedAction),
      switchMap((action) => {

      const btcPayInvoiceModalState$
          = this.store.pipe(select(selectBtcPayInvoiceModalState));

      return combineLatest([btcPayInvoiceModalState$]).pipe(first(),
                                                                   map(([btcPayInvoiceModalState]) => {
              if (btcPayInvoiceModalState.hasPaymentToInformUserAbout){
                this.translate.get('REGISTER_BTC_CHAT_EFFECT.INVOICE_PAYED')
                  .subscribe(message => {
                    this.snackBar.open(message, "", {duration: 2000});
                    this.registerBtcChatFormResetService
                        .resetBtcChatForm
                        .next(true);
                  });
              } else {
                // this.translate.get('REGISTER_BTC_CHAT_EFFECT.INVOICE_CANCELED')
                //   .subscribe(message => {
                //     this.snackBar.open(message, "", {duration: 2000});
                //   });
              }
              return InformUserAboutPayedInvoiceAction({payload: {}})
            }));
      })
    )
  );


}
