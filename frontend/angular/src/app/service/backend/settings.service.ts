import { Injectable } from '@angular/core';
import {HttpClient} from "@angular/common/http";
import {EndpointService} from "./endpoint.service";
import {Observable} from "rxjs";
import {IWebhookTestResult} from "../../model/settings/webhook-test-result";
import {IStoreWebhookActivation} from "../../model/settings/store-webhook-activation.ts/store-webhook-activation";
import {IBtcPayConfig} from "../../model/settings/btc-pay/btc-pay-config";
import {IStoreWebhookConfig} from "../../model/settings/store-webhook-config/store-webhook-config";

@Injectable({
  providedIn: 'root'
})
export class SettingsService {

  constructor(private readonly _http: HttpClient,
              private readonly _backend: EndpointService) {

  }

  public getStoreWebhookActivation(): Observable<IStoreWebhookActivation> {
    return this._http.get<IStoreWebhookActivation>(this._backend.getApiBase() + "/settings/storewebhookactivation");
  }

  public patchStoreWebhookActivation(storeWebhookActivation: IStoreWebhookActivation): Observable<{}> {
    return this._http.patch<{ }>(this._backend.getApiBase() + "/settings/storewebhookactivation", storeWebhookActivation);
  }

  public getStoreWebhookConfig(): Observable<IStoreWebhookConfig> {
    return this._http.get<IStoreWebhookConfig>(this._backend.getApiBase() + "/settings/storewebhookconfig");
  }

  public patchStoreWebhookConfig(storeWebhookConfigWrapper: IStoreWebhookConfig): Observable<{}> {
    return this._http.patch<{ }>(this._backend.getApiBase() + "/settings/storewebhookconfig", storeWebhookConfigWrapper);
  }

  public postTestWebhook() : Observable<IWebhookTestResult> {
    return this._http.post<IWebhookTestResult>(this._backend.getApiBase() + "/settings/teststorewebhook",
      {});
  }

  public getBtcPayConfig() : Observable<IBtcPayConfig> {
    return this._http.get<IBtcPayConfig>(this._backend.getApiBase() + "/settings/btcpayconfig");
  }

  public postResetBtcPayPassword(): Observable<{ }> {
    return this._http.post<{ }>(this._backend.getApiBase() + "/settings/resetbtcpaypassword",
  {});
  }

  public postDeleteAccount(): Observable<{ }> {
    return this._http.post<{ }>(this._backend.getApiBase() + "/settings/deleteaccount",
      {});
  }
}
