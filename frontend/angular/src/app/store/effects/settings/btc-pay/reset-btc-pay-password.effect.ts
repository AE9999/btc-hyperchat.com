import {Injectable} from "@angular/core";
import {Actions, createEffect, ofType} from "@ngrx/effects";
import {MatSnackBar} from "@angular/material/snack-bar";
import {TranslateService} from "@ngx-translate/core";
import {
  RequestBtcPayPasswordResetAction,
  RequestBtcPayPasswordResetResponseOkAction,
  RequestBtcPayPasswordResetResponseErrorAction
} from "../../../actions/settings/btc-pay/reset-btc-pay-password.actions";
import {catchError, map, of, switchMap} from "rxjs";
import {SettingsService} from "../../../../service/backend/settings.service";

@Injectable()
export class ResetBtcPayPasswordEffect {
  constructor(private readonly actions: Actions,
              private readonly snackBar: MatSnackBar,
              private settingsService : SettingsService,
              private translate: TranslateService) {
  }

  successfulDelete(_response: {}) {
    this.translate.get('SETTINGS_EFFECT.RESET_PASSWORD_OK')
      .subscribe(message => {
        this.snackBar.open(message, "", {duration: 2000});
      });

    return RequestBtcPayPasswordResetAction({payload: { }});
  }

  resetBtcPassword$ = createEffect(() =>
    this.actions.pipe(
      ofType(RequestBtcPayPasswordResetAction),
      switchMap((action)  => {
        return this.settingsService.postResetBtcPayPassword().pipe(
          map((btcPayConfig) => RequestBtcPayPasswordResetResponseOkAction({payload: {btcPayConfig: btcPayConfig}})),
          catchError((error) => {
            this.translate.get('SETTINGS_EFFECT.RESET_PASSWORD_ERROR')
              .subscribe(message =>
                this.snackBar.open(message, "",{duration: 2000})
              );
            return of(RequestBtcPayPasswordResetResponseErrorAction({payload: {}}))
          }),
        )
      }),
    )
  );

}
