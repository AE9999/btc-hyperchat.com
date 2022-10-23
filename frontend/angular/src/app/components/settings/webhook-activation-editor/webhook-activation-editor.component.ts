import { Component, OnInit } from '@angular/core';
import {combineLatest, distinctUntilChanged, filter, Observable, of} from "rxjs";
import {FormGroup, FormBuilder} from "@angular/forms";
import {
  IGetStoreWebhookConfigState
} from "../../../store/reducers/settings/webhook/store-webhook-config/get-store-webhook-config.reducer";
import {select, Store} from "@ngrx/store";
import {
  AppState, selectGetStoreWebhookActivationState,
  selectGetStoreWebhookConfigState,
  selectUpdateStoreWebhookActivationState
} from "../../../store/reducers";
import {
  IUpdateStoreWebhookActivationState
} from "../../../store/reducers/settings/webhook/store-webhook-activation/update-store-webhook-activation.reducer";
import {TranslateService} from "@ngx-translate/core";
import {first} from "rxjs/operators";
import {
  RequestUpdateStoreWebhookActivationAction
} from "../../../store/actions/settings/webhook/store-webhook-activation/update-store-webhook-activation.action";
import {IStoreWebhookActivation} from "../../../model/settings/store-webhook-activation.ts/store-webhook-activation";
import {isEqual} from "lodash";
import {
  RequestGetStoreWebhookActivationAction
} from "../../../store/actions/settings/webhook/store-webhook-activation/get-store-webhook-activation.action";
import {
  IGetStoreWebhookActivationState
} from "../../../store/reducers/settings/webhook/store-webhook-activation/get-store-webhook-activation.reducer";

@Component({
  selector: 'app-webhook-activation-editor',
  templateUrl: './webhook-activation-editor.component.html',
  styleUrls: ['./webhook-activation-editor.component.css']
})
export class WebhookActivationEditorComponent implements OnInit {

  noValidWebhook$: Observable<boolean>;

  updating$: Observable<boolean>;

  error$: Observable<boolean>;

  loading$: Observable<boolean>;

  formGroup: FormGroup;

  tooltip_disabled: string = '';

  tooltip_disable_webhook: string = '';

  tooltip_automatically_process: string = '';

  tooltip_save: string = '';

  constructor(private store: Store<AppState>,
              private formBuilder: FormBuilder,
              private translate: TranslateService) {

    let getStoreWebhookConfigState$: Observable<IGetStoreWebhookConfigState> =
      store.pipe(select(selectGetStoreWebhookConfigState));

    let getStoreWebhookActivationState$: Observable<IGetStoreWebhookActivationState> =
      store.pipe(select(selectGetStoreWebhookActivationState));

    let updateStoreWebhookActivationState$: Observable<IUpdateStoreWebhookActivationState> =
      store.pipe(select(selectUpdateStoreWebhookActivationState));

    this.translate.get('SETTINGS.ACTIVATION_TAB_TOOLTIP')
      .subscribe(value => this.tooltip_disabled = value);

    this.translate.get('SETTINGS.ACTIVATION_TAB_TOOLTIP_DISABLE_WEBHOOK')
      .subscribe(value => this.tooltip_disable_webhook = value);

    this.translate.get('SETTINGS.ACTIVATION_TAB_TOOLTIP_AUTOMATICALLY_PROCESS')
      .subscribe(value => this.tooltip_automatically_process = value);

    this.translate.get('SETTINGS.ACTIVATION_TAB_TOOLTIP_SAVE')
      .subscribe(value => this.tooltip_save = value);

    this.updating$ =
      updateStoreWebhookActivationState$.pipe(select( state => state.loading));

    this.error$ =
      getStoreWebhookActivationState$.pipe(select(state => state.error));

    this.loading$ =
      getStoreWebhookActivationState$.pipe(select(state => state.loading));

    this.noValidWebhook$ =
      getStoreWebhookConfigState$
        .pipe(select(state => !state.storeWebhookConfig
                                     || !(state.storeWebhookConfig.url)
                                     || !(state.storeWebhookConfig.postTypeCode)));

    this.formGroup = formBuilder.group({
      webhookActive: [false],
      automaticallyProcessBtcChatsIfWebhookSucceeds: [false],
    });

    this.noValidWebhook$.subscribe(value => {
      if (!value) {
        this.formGroup.get('webhookActive')?.enable();
        this.formGroup.get('automaticallyProcessBtcChatsIfWebhookSucceeds')?.enable();
      } else {
        this.formGroup.get('webhookActive')?.disable();
        this.formGroup.get('automaticallyProcessBtcChatsIfWebhookSucceeds')?.disable();
      }
    })

    let storeWebhookActivation$ : Observable<IStoreWebhookActivation | undefined> =
      getStoreWebhookActivationState$.
      pipe(select(state => state.storeWebhookActivation),
           filter(storeWebhookActivation => storeWebhookActivation != undefined),
           distinctUntilChanged((prev, curr) => isEqual(prev, curr)));

    storeWebhookActivation$.subscribe(storeWebhookActivation => {
      if (storeWebhookActivation) {
        this.formGroup.patchValue(storeWebhookActivation)
      }
    })

  }

  updateSettings() {
    combineLatest([this.updating$])
      .pipe(first()).subscribe(([updating]) =>  {

      if (updating) {
        return;
      }

      const storeWebhookActivation : IStoreWebhookActivation =  {
        webhookActive:
          !!(this.formGroup.get('webhookActive')?.value),
        automaticallyProcessBtcChatsIfWebhookSucceeds:
          !!(this.formGroup.get('automaticallyProcessBtcChatsIfWebhookSucceeds')?.value),
      };

      const payload = { payload: { storeWebhookActivation: storeWebhookActivation } };

      this.store
        .dispatch(RequestUpdateStoreWebhookActivationAction(payload))
    });
  }

  ngOnInit(): void {
    this.store
      .dispatch(RequestGetStoreWebhookActivationAction({ payload: {}}));

  }

}
