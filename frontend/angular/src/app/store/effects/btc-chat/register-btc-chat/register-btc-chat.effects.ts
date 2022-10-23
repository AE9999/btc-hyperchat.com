import {Injectable} from "@angular/core";
import {Actions, createEffect, ofType} from "@ngrx/effects";
import {
  RegisterBtcChatRequestAction,
  RegisterBtcChatResponseOkAction,
  RegisterBtcChatResponseErrorAction
} from "../../../actions/btc-chat/register-btc-chat/register-btc-chat.actions";
import {catchError, map, of, switchMap} from "rxjs";
import {MatSnackBar} from "@angular/material/snack-bar";
import {BtcChatService} from "../../../../service/backend/btc-chat.service";
import {TranslateService} from "@ngx-translate/core";
import {IRegisterBtcChatResponse} from "../../../../model/btc-chat/register-btc-chat/register-btc-chat-response";
import {IRegisterBtcChatRequest} from "../../../../model/btc-chat/register-btc-chat/register-btc-chat-request";

@Injectable()
export class RegisterBtcChatEffects {

  constructor(
    private readonly actions: Actions,
    private readonly snackBar: MatSnackBar,
    private btcChatService: BtcChatService,
    private translate: TranslateService,
  ) {
  }


  callEndpoint(registerBtcChatRequest : IRegisterBtcChatRequest) {
    this.translate
        .get('REGISTER_BTC_CHAT_EFFECT.SENDING_MESSAGE')
        .subscribe(message =>
        this.snackBar.open(message, "",{duration: 2000})
      );
    return this.btcChatService
               .postRegisterBtcChatRequest(registerBtcChatRequest)
  }

  registerBtcChat$ = createEffect(() =>
    this.actions.pipe(
      ofType(RegisterBtcChatRequestAction),
      switchMap((action) =>
          this.callEndpoint(action.payload.registerBtcChatRequest)
          .pipe(map((registerBtcChatResponse) =>
                    RegisterBtcChatResponseOkAction(
                      { payload: { registerBtcChatResponse: registerBtcChatResponse } })),
            catchError((error) => {
              this.translate.get('REGISTER_BTC_CHAT_EFFECT.ERROR')
                .subscribe(message =>
                  this.snackBar.open(message, "",{duration: 2000})
                );
              return of(RegisterBtcChatResponseErrorAction({payload: {}}))
            }),
          )
      )
    )
  );
}
