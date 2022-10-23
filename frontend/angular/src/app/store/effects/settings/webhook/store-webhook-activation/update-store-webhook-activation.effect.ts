import {Injectable} from "@angular/core";
import {Actions, createEffect, ofType} from "@ngrx/effects";
import {MatSnackBar} from "@angular/material/snack-bar";

import {TranslateService} from "@ngx-translate/core";
import {SettingsService} from "../../../../../service/backend/settings.service";
import {
  RequestUpdateStoreWebhookActivationAction,
  RequestUpdateStoreWebhookActivationOkAction,
  RequestUpdateStoreWebhookActivationErrorAction
} from "../../../../actions/settings/webhook/store-webhook-activation/update-store-webhook-activation.action";
import {catchError, map, of, switchMap} from "rxjs";

@Injectable()
export class UpdateStoreWebhookActivationEffects {
  constructor(private readonly actions: Actions,
              private readonly snackBar: MatSnackBar,
              private settingsService: SettingsService,
              private translate: TranslateService) {}

  succesfullPatch(_response: {}) {
    this.translate.get('SETTINGS_EFFECT.PATCHING_SUCCESSFUL')
      .subscribe(message =>
        this.snackBar.open(message, "",{duration: 2000})
      );

    return RequestUpdateStoreWebhookActivationOkAction({payload: { }});
  }

  patchSettings$ = createEffect(() =>
    this.actions.pipe(
      ofType(RequestUpdateStoreWebhookActivationAction),
      switchMap((action)  => {
          return this.settingsService.patchStoreWebhookActivation(action.payload.storeWebhookActivation).pipe(
            map((response) => this.succesfullPatch(response)),
            catchError((error) => {
              this.translate.get('SETTINGS_EFFECT.PATCHING_ERROR')
                .subscribe(message =>
                  this.snackBar.open(message, "",{duration: 2000})
                );
              return of(RequestUpdateStoreWebhookActivationErrorAction({payload: {}}))
            }),
          )
        }
      )
    )
  );

}
