import {Injectable} from "@angular/core";
import {Actions, createEffect, ofType} from "@ngrx/effects";
import {MatSnackBar} from "@angular/material/snack-bar";

import {TranslateService} from "@ngx-translate/core";
import {SettingsService} from "../../../../../service/backend/settings.service";
import {
  RequestGetStoreWebhookConfigAction,
  RequestGetStoreWebhookConfigOkAction,
  RequestGetStoreWebhookConfigErrorAction
} from "../../../../actions/settings/webhook/store-webhook-config/get-store-webhook-config";
import {catchError, map, of, switchMap} from "rxjs";

@Injectable()
export class GetStoreWebhookConfigEffects {
  constructor(private readonly actions: Actions,
              private readonly snackBar: MatSnackBar,
              private settingsService: SettingsService,
              private translate: TranslateService) {}

  loadSettings$ = createEffect(() =>
    this.actions.pipe(
      ofType(RequestGetStoreWebhookConfigAction),
      switchMap((action)  => {
        return this.settingsService.getStoreWebhookConfig().pipe(
          map((storeWebhookConfig) => RequestGetStoreWebhookConfigOkAction({payload: {storeWebhookConfig: storeWebhookConfig}})),
          catchError((error) => {

            this.translate.get('SETTINGS_EFFECT.LOADING_ERROR')
              .subscribe(message =>
                this.snackBar.open(message, "",{duration: 2000})
              );
            return of(RequestGetStoreWebhookConfigErrorAction({payload: {}}))
          }),
        )
      }),
    )
  );

}
