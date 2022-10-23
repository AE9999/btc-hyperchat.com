import {Injectable} from "@angular/core";
import {Actions, createEffect, ofType} from "@ngrx/effects";
import {MatSnackBar} from "@angular/material/snack-bar";

import {TranslateService} from "@ngx-translate/core";
import {SettingsService} from "../../../../../service/backend/settings.service";
import {
  RequestUpdateStoreWebhookConfigAction,
  RequestUpdateStoreWebhookConfigOkAction,
  RequestUpdateStoreWebhookConfigErrorAction
} from "../../../../actions/settings/webhook/store-webhook-config/update-store-webhook-config";
import {catchError, map, of, switchMap} from "rxjs";
import {IStoreWebhookConfig} from "../../../../../model/settings/store-webhook-config/store-webhook-config";

@Injectable()
export class UpdateStoreWebhookConfigEffects {
  constructor(private readonly actions: Actions,
              private readonly snackBar: MatSnackBar,
              private settingsService: SettingsService,
              private translate: TranslateService) {}

  succesfullPatch(_response: {},
                  storeWebhookConfig: IStoreWebhookConfig) {
    this.translate.get('SETTINGS_EFFECT.PATCHING_SUCCESSFUL')
      .subscribe(message =>
        this.snackBar.open(message, "",{duration: 2000})
      );

    return RequestUpdateStoreWebhookConfigOkAction({payload: { storeWebhookConfig }});
  }

  patchSettings$ = createEffect(() =>
    this.actions.pipe(
      ofType(RequestUpdateStoreWebhookConfigAction),
      switchMap((action)  => {
          return this.settingsService.patchStoreWebhookConfig(action.payload.storeWebhookConfig).pipe(
            map((response) => this.succesfullPatch(response,
                                                             action.payload.storeWebhookConfig)),
            catchError((error) => {
              this.translate.get('SETTINGS_EFFECT.PATCHING_ERROR')
                .subscribe(message =>
                  this.snackBar.open(message, "",{duration: 2000})
                );
              return of(RequestUpdateStoreWebhookConfigErrorAction({payload: {}}))
            }),
          )
        }
      )
    )
  );

}
