import {Injectable} from "@angular/core";
import {Actions, createEffect, ofType} from "@ngrx/effects";
import {MatSnackBar} from "@angular/material/snack-bar";

import {TranslateService} from "@ngx-translate/core";
import {SettingsService} from "../../../../service/backend/settings.service";
import {
  RequestWebhookTestAction,
  RequestWebhookResponseErrorAction,
  RequestWebhookTestResponseFailureAction,
  RequestWebhookTestResponseOkAction
} from "../../../actions/settings/webhook/test-webhook.action";
import {catchError, map, of, switchMap} from "rxjs";
import {IWebhookTestResult} from "../../../../model/settings/webhook-test-result";

@Injectable()
export class TestWebhookEffects {
  constructor(private readonly actions: Actions,
              private readonly snackBar: MatSnackBar,
              private settingsService: SettingsService,
              private translate: TranslateService) {}

  parseWebhookResult(webhookTestResult: IWebhookTestResult) {
    if (webhookTestResult.success) {
      this.translate.get('SETTINGS_EFFECT.WEBHOOK_TEST_OK')
        .subscribe(message =>
          this.snackBar.open(message, "", {duration: 2000})
        );
      return RequestWebhookTestResponseOkAction({payload: {}});
    } else {
      this.translate.get('SETTINGS_EFFECT.WEBHOOK_TEST_FAILURE')
        .subscribe(message =>
          this.snackBar.open(message, "", {duration: 2000})
        );
      return RequestWebhookTestResponseFailureAction({payload: {}});
    }
  }

  testWebhook$ = createEffect(() =>
    this.actions.pipe(
      ofType(RequestWebhookTestAction),
      switchMap((action)  => {
        return this.settingsService.postTestWebhook().pipe(
          map((webhookTestResult) => this.parseWebhookResult(webhookTestResult)),

          catchError((error) => {
            this.translate.get('SETTINGS_EFFECT.WEBHOOK_TEST_ERROR')
              .subscribe(message =>
                this.snackBar.open(message, "",{duration: 2000})
              );
            return of(RequestWebhookResponseErrorAction({payload: {}}))
          }),
        )
      }),
    )
  );
}
