import {Injectable} from "@angular/core";
import {Actions, createEffect, ofType} from "@ngrx/effects";
import {MatSnackBar} from "@angular/material/snack-bar";
import {TranslateService} from "@ngx-translate/core";
import {
  RequestBtcPayConfigAction,
  RequestBtcPayConfigResponseOkAction,
  RequestBtcPayConfigResponseErrorAction
} from "../../../actions/settings/btc-pay/btc-pay-config.actions";
import {catchError, map, of, switchMap} from "rxjs";
import {SettingsService} from "../../../../service/backend/settings.service";

@Injectable()
export class BtcPayConfigEffects {
  constructor(private readonly actions: Actions,
              private readonly snackBar: MatSnackBar,
              private settingsService : SettingsService,
              private translate: TranslateService) {
  }

  getBtcPayConfig$ = createEffect(() =>
    this.actions.pipe(
      ofType(RequestBtcPayConfigAction),
      switchMap((action)  => {
        return this.settingsService.getBtcPayConfig().pipe(
          map((btcPayConfig) => RequestBtcPayConfigResponseOkAction({payload: {btcPayConfig: btcPayConfig}})),
          catchError((error) => {
            this.translate.get('SETTINGS_EFFECT.GET_BTC_PAY_CONFIG_ERROR')
              .subscribe(message =>
                this.snackBar.open(message, "",{duration: 2000})
              );
            return of(RequestBtcPayConfigResponseErrorAction({payload: {}}))
          }),
        )
      }),
    )
  );

}
