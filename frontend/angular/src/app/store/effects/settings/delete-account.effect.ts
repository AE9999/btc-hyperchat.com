import {Injectable} from "@angular/core";
import {Actions, createEffect, ofType} from "@ngrx/effects";
import {MatSnackBar} from "@angular/material/snack-bar";
import {TranslateService} from "@ngx-translate/core";
import {
  RequestDeleteAccountAction,
  RequestDeleteAccountResponseOkAction,
  RequestDeleteAccountResponseErrorAction
} from "../../actions/settings/delete-account.action";
import {catchError, map, of, switchMap} from "rxjs";
import {SettingsService} from "../../../service/backend/settings.service";
import {KeycloakService} from "keycloak-angular";

@Injectable()
export class DeleteAccountEffects {
  constructor(private readonly actions: Actions,
              private readonly snackBar: MatSnackBar,
              private settingsService : SettingsService,
              private keycloakService: KeycloakService,
              private translate: TranslateService) {
  }

  successfulDelete(_response: {}) {
    this.translate.get('SETTINGS_EFFECT.ACCOUNT_DELETED_OK')
      .subscribe(message => {
        this.snackBar.open(message, "", {duration: 2000});
        this.keycloakService.logout();
      });

    return RequestDeleteAccountResponseOkAction({payload: { }});
  }

  deleteAccount$ = createEffect(() =>
    this.actions.pipe(
      ofType(RequestDeleteAccountAction),
      switchMap((action)  => {
        return this.settingsService.postDeleteAccount().pipe(
          map((ok) => this.successfulDelete(ok)),
          catchError((error) => {
            this.translate.get('SETTINGS_EFFECT.DELETE_ACCOUNT_ERROR')
              .subscribe(message =>
                this.snackBar.open(message, "",{duration: 2000})
              );
            return of(RequestDeleteAccountResponseErrorAction({payload: {}}))
          }),
        )
      }),
    )
  );

}
