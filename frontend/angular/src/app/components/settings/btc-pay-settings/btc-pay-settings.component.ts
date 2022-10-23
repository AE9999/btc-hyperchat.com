import { Component, OnInit } from '@angular/core';
import {TranslateService} from "@ngx-translate/core";
import {combineLatest, map, Observable, of, Subject} from "rxjs";
import {select, Store} from "@ngrx/store";
import {
  AppState,
  selectGetBtcPayConfigState,
  selectGetStoreWebhookConfigState,
  selectResetBtcPayPasswordState
} from "../../../store/reducers";
import {RequestBtcPayConfigAction} from "../../../store/actions/settings/btc-pay/btc-pay-config.actions";
import {RequestBtcPayPasswordResetAction} from "../../../store/actions/settings/btc-pay/reset-btc-pay-password.actions";
import {IGetBtcPayConfigState} from "../../../store/reducers/settings/btc-pay/btc-pay-config.reducer";
import {IResetBtcPayPasswordState} from "../../../store/reducers/settings/btc-pay/reset-btc-pay-password.reducer";

@Component({
  selector: 'app-btc-pay-settings',
  templateUrl: './btc-pay-settings.component.html',
  styleUrls: ['./btc-pay-settings.component.css']
})
export class BtcPaySettingsComponent implements OnInit {

  username$: Observable<string | undefined>;

  initial_password$: Observable<string | undefined>;

  btcPayUrl$: Observable<string | undefined>;

  error$: Observable<boolean>;

  leading$ : Observable<boolean>;

  resettingPassword$: Observable<boolean>;

  errorPaaswordReset$ : Observable<boolean>

  tooltip_btc_reset_account: string  = '';

  credentials: string = '';

  constructor(private store: Store<AppState>,
              private translate: TranslateService) {

    this.translate.get('SETTINGS.BTCPAY_RESET_ACCOUNT_TOOLTIP')
      .subscribe(value => this.tooltip_btc_reset_account = value);

    const getBtcPayConfigState$: Observable<IGetBtcPayConfigState> =
      store.pipe(select(selectGetBtcPayConfigState));

    this.leading$ =
      getBtcPayConfigState$.pipe(select(state => state.loading));

    this.error$ =
      getBtcPayConfigState$.pipe(select(state => state.error));

    this.username$ =
      getBtcPayConfigState$.pipe(select(state => state.btcPayConfig?.username));

    this.initial_password$ =
      getBtcPayConfigState$.pipe(select(state => state.btcPayConfig?.initialPassword));

    this.btcPayUrl$ =
      getBtcPayConfigState$.pipe(select(state => state.btcPayConfig?.serverUrl));

    const resetBtcPayPasswordState$: Observable<IResetBtcPayPasswordState> =
      store.pipe(select(selectResetBtcPayPasswordState));;

    this.resettingPassword$ =
      resetBtcPayPasswordState$.pipe(select(state => state.loading));

    this.errorPaaswordReset$ =
      resetBtcPayPasswordState$.pipe(select(state => state.error));

  }

  resetAccount() {
    this.store.dispatch(RequestBtcPayPasswordResetAction({ payload: {}}))
  }

  ngOnInit(): void {
    this.store.dispatch(RequestBtcPayConfigAction({payload: {}}))

    let me = this;

    // Insanly lazy improve, something with flatmap probably
    combineLatest([this.username$, this.initial_password$])
      .subscribe(([username, initialPassword]) => {
        this.translate.get('SETTINGS.BTCPAY_ACCOUNT_SUMMARY',
          {username: username, initialPassword: initialPassword})
          .subscribe(x => me.credentials = x);
      });
  }

}
