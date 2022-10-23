import {Injectable} from "@angular/core";
import {Actions, createEffect, ofType} from "@ngrx/effects";
import {MatSnackBar} from "@angular/material/snack-bar";
import {BtcChatService} from "../../../../service/backend/btc-chat.service";
import {TranslateService} from "@ngx-translate/core";
import {IProcessBtcChatResponse} from "../../../../model/btc-chat/process-btc-chat/process-btc-chat-response";
import {catchError, map, of, switchMap} from "rxjs";
import {
  RequestProcessBtcChatAction,
  RequestProcessBtcChatResponseOkAction,
  RequestProcessBtcChatResponseErrorAction
} from "../../../actions/btc-chat/my-received-btc-chats/process-btc-chat.action"


@Injectable()
export class ProcessBtcChatEffects {
  constructor(
    private readonly actions: Actions,
    private readonly snackBar: MatSnackBar,
    private btcChatService: BtcChatService,
    private translate: TranslateService,
  ) {
  }

  process_ok(processBtcChatResponse: IProcessBtcChatResponse) {
    this.translate.get('MY_RECEIVED_BTC_CHATS_EFFECT.PROCESS_MY_RECEIVED_BTC_CHAT_OK')
      .subscribe(message =>
        this.snackBar.open(message, "",{duration: 2000})
      );

    return RequestProcessBtcChatResponseOkAction({payload: {processBtcChatResponse: processBtcChatResponse}});
  }

  processBtcChat$ = createEffect(() =>
    this.actions.pipe(
      ofType(RequestProcessBtcChatAction),
      switchMap((action) => {
        return this.btcChatService
          .postProcessBtcChatRequest(action.payload.processBtcChatRequest)
          .pipe(
            map((processBtcChatResponse) => this.process_ok(processBtcChatResponse),
              catchError((error) => {
                this.translate.get('MY_RECEIVED_BTC_CHATS_EFFECT.PROCESS_MY_RECEIVED_BTC_CHAT_ERROR')
                  .subscribe(message =>
                    this.snackBar.open(message, "",{duration: 2000})
                  );
                let payload = {
                  payload: {
                    processBtcChatRequest: action.payload.processBtcChatRequest
                  }
                };
                return of(RequestProcessBtcChatResponseErrorAction(payload))
              }),
            ));
      })
    )
  );
}
