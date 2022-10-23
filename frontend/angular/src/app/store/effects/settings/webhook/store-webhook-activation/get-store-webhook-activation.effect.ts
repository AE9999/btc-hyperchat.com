import {Injectable} from "@angular/core";
import {Actions, createEffect, ofType} from "@ngrx/effects";
import {MatSnackBar} from "@angular/material/snack-bar";

import {TranslateService} from "@ngx-translate/core";
import {SettingsService} from "../../../../../service/backend/settings.service";
import {
  RequestGetStoreWebhookActivationErrorAction,
  RequestGetStoreWebhookActivationOkAction,
  RequestGetStoreWebhookActivationAction
} from "../../../../actions/settings/webhook/store-webhook-activation/get-store-webhook-activation.action";
import {catchError, map, of, switchMap} from "rxjs";

@Injectable()
export class GetStoreWebhookActivationEffects {
  constructor(private readonly actions: Actions,
              private readonly snackBar: MatSnackBar,
              private settingsService: SettingsService,
              private translate: TranslateService) {}

  loadSettings$ = createEffect(() =>
    this.actions.pipe(
      ofType(RequestGetStoreWebhookActivationAction),
      switchMap((action)  => {
        return this.settingsService.getStoreWebhookActivation().pipe(
          map((storeWebhookActivation) => RequestGetStoreWebhookActivationOkAction({payload: {storeWebhookActivation: storeWebhookActivation}})),
          catchError((error) => {

            this.translate.get('SETTINGS_EFFECT.LOADING_ERROR')
              .subscribe(message =>
                this.snackBar.open(message, "",{duration: 2000})
              );
            return of(RequestGetStoreWebhookActivationErrorAction({payload: {}}))
          }),
        )
      }),
    )
  );

}
