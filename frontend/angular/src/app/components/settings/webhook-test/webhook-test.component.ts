import { Component, OnInit } from '@angular/core';
import {select, Store} from "@ngrx/store";
import {
  AppState,
  selectGetStoreWebhookConfigState,
  selectTestWebhookState
} from "../../../store/reducers";
import {combineLatest, Observable} from "rxjs";
import {first} from "rxjs/operators";
import {RequestWebhookTestAction} from "../../../store/actions/settings/webhook/test-webhook.action";
import {ITestWebhookState} from "../../../store/reducers/settings/webhook/test-webhook.reducer";
import {
  IGetStoreWebhookConfigState
} from "../../../store/reducers/settings/webhook/store-webhook-config/get-store-webhook-config.reducer";
import {IStoreWebhookConfig} from "../../../model/settings/store-webhook-config/store-webhook-config";
import {TranslateService} from "@ngx-translate/core";

@Component({
  selector: 'app-webhook-test',
  templateUrl: './webhook-test.component.html',
  styleUrls: ['./webhook-test.component.css']
})
export class WebhookTestComponent implements OnInit {

  testingWebhook$: Observable<boolean>;

  noValidWebhook$: Observable<boolean>;

  tooltip_disabled: string = '';

  tooltip_test_button: string = '';

  constructor(private store: Store<AppState>,
              private translate: TranslateService) {

    let getStoreWebhookConfigState$: Observable<IGetStoreWebhookConfigState> =
      store.pipe(select(selectGetStoreWebhookConfigState));

    let testWebhookState$: Observable<ITestWebhookState> =
      store.pipe(select(selectTestWebhookState))

    this.translate.get('SETTINGS.TEST_TAB_TOOLTIP')
      .subscribe(value => this.tooltip_disabled = value);

    this.translate.get('SETTINGS.TEST_TAB_TOOLTIP_TEST_BUTTON')
      .subscribe(value => this.tooltip_test_button = value);

    this.testingWebhook$ =
      testWebhookState$.pipe(select(state => state.loading))

    this.noValidWebhook$ =
      getStoreWebhookConfigState$
        .pipe(select(state => !state.storeWebhookConfig
          || !(state.storeWebhookConfig.url)
          || !(state.storeWebhookConfig.postTypeCode)));
  }

  ngOnInit(): void {
  }

  testWebhook() {
    combineLatest([this.testingWebhook$])
      .pipe(first()).subscribe(([testingWebhook]) =>  {

      if (testingWebhook) {
        return;
      }

      this.store
        .dispatch(RequestWebhookTestAction({payload: {  }}))
    });
  }

}
