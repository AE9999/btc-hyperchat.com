import {Injectable} from "@angular/core";
import {Actions, createEffect, ofType} from "@ngrx/effects";
import {MatSnackBar} from "@angular/material/snack-bar";
import {TranslateService} from "@ngx-translate/core";
import {BtcChatService} from "../../../../service/backend/btc-chat.service";
import {
  RequestGetTestBtcChatAction,
  RequestGetTestBtcChatOkAction,
  RequestGetTestBtcChatErrorAction
} from "../../../actions/settings/webhook/get-test-btc-chat.action";
import {catchError, map, of, switchMap} from "rxjs";

@Injectable()
export class GetTestBtcChatEffects {
  constructor(private readonly actions: Actions,
              private readonly snackBar: MatSnackBar,
              private btcChatService: BtcChatService,
              private translate: TranslateService) {
  }

  loadSettings$ = createEffect(() =>
    this.actions.pipe(
      ofType(RequestGetTestBtcChatAction),
      switchMap((action)  => {
        return this.btcChatService.getTestBtcChat().pipe(
          map((myReceivedBtcChat) => RequestGetTestBtcChatOkAction({payload: {myReceivedBtcChat: myReceivedBtcChat}})),
          catchError((error) => {
            this.translate.get('SETTINGS_EFFECT.LOADING_TEST_BTC_CHAT_ERROR')
              .subscribe(message =>
                this.snackBar.open(message, "",{duration: 2000})
              );
            return of(RequestGetTestBtcChatErrorAction({payload: {}}))
          }),
        )
      }),
    )
  );

}
