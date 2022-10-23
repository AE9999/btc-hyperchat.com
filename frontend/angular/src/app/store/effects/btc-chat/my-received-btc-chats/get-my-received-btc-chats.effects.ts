import {Injectable} from "@angular/core";
import {Actions, createEffect, ofType} from "@ngrx/effects";
import {MatSnackBar} from "@angular/material/snack-bar";
import {Store} from "@ngrx/store";
import {AppState} from "../../../reducers";
import {BtcChatService} from "../../../../service/backend/btc-chat.service";
import {TranslateService} from "@ngx-translate/core";
import {catchError, map, of, switchMap} from "rxjs";
import {
  RequestGetMyReceivedBtcChatsAction,
  RequestGetMyReceivedBtcChatsResponseErrorAction,
  RequestGetMyReceivedBtcChatsResponseOkAction
} from "../../../actions/btc-chat/my-received-btc-chats/get-my-received-btc-chats.action";

@Injectable()
export class GetMyReceivedBtcChatsEffects {

  constructor(
    private readonly actions: Actions,
    private readonly snackBar: MatSnackBar,
    private store: Store<AppState>,
    private btcChatService: BtcChatService,
    private translate: TranslateService,
  ) {
  }

  getMyReceivedBtcChats$ = createEffect(() =>
    this.actions.pipe(
      ofType(RequestGetMyReceivedBtcChatsAction),
      switchMap((action) => {
        return this.btcChatService
          .getCurrentBtcChats()
          .pipe(
            map((myReceivedBtcChatsResponse) =>
              RequestGetMyReceivedBtcChatsResponseOkAction({payload: {myReceivedBtcChatsResponse: myReceivedBtcChatsResponse}})),
            catchError((error) => {
              this.translate.get('MY_RECEIVED_BTC_CHATS_EFFECT.QUERY_ERROR')
                .subscribe(message =>
                  this.snackBar.open(message, "",{duration: 2000})
                );
              return of(RequestGetMyReceivedBtcChatsResponseErrorAction({payload: {}}))
            }),
          );
      })
    )
  );

}
