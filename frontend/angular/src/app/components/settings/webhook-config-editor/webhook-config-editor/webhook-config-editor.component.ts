import { Component, OnInit, ViewChild} from '@angular/core';
import {select, Store} from "@ngrx/store";
import {
  AppState,
  selectGetStoreWebhookConfigState, selectGetTestBtcChatState,
  selectUpdateStoreWebhookConfigState,
} from "../../../../store/reducers";
import {combineLatest, map, Observable, ReplaySubject } from "rxjs";
import {
  IGetStoreWebhookConfigState
} from "../../../../store/reducers/settings/webhook/store-webhook-config/get-store-webhook-config.reducer";
import {
  IUpdateStoreWebhookConfigState
} from "../../../../store/reducers/settings/webhook/store-webhook-config/update-store-webhook-config.reducer";
import {
  RequestGetStoreWebhookConfigAction
} from "../../../../store/actions/settings/webhook/store-webhook-config/get-store-webhook-config";
import {IGetTestBtcChatState} from "../../../../store/reducers/settings/webhook/get-test-btc-chat.reducer";
import {DynamicListComponent} from "../dynamic-list/dynamic-list.component";
import {RequestGetTestBtcChatAction} from "../../../../store/actions/settings/webhook/get-test-btc-chat.action";
import {CurlDemoComponent} from "../curl-demo/curl-demo.component";
import {UrlInputComponent} from "../url-input/url-input.component";
import {PostTypeInputComponent} from "../post-type-input/post-type-input.component";
import {TranslateService} from "@ngx-translate/core";
import {IStoreWebhookConfig} from "../../../../model/settings/store-webhook-config/store-webhook-config";
import {first} from "rxjs/operators";
import {
  RequestUpdateStoreWebhookConfigAction
} from "../../../../store/actions/settings/webhook/store-webhook-config/update-store-webhook-config";
import {IMyReceivedBtcChat} from "../../../../model/btc-chat/my-received-btc-chats/my-received-btc-chat";


@Component({
  selector: 'app-webhook-config-editor',
  templateUrl: './webhook-config-editor.component.html',
  styleUrls: ['./webhook-config-editor.component.css']
})
export class WebhookConfigEditorComponent implements OnInit {

  // Thanks. https://tinytip.co/tips/angular-output-observable/

  // TODO: Check if replay subjects can be replaced with behavior subjects as we are only interested in the latest value

  updating$: Observable<boolean>;

  loading$: Observable<boolean>;

  error$: Observable<boolean>;

  exampleBtcChatString$: Observable<string>;

  exampleBtcChat$: Observable<IMyReceivedBtcChat | undefined>;

  @ViewChild('urlInput')
  urlInput: UrlInputComponent | undefined;

  @ViewChild('postType')
  postType: PostTypeInputComponent | undefined

  @ViewChild('headerConfig')
  headerConfig : DynamicListComponent | undefined;

  @ViewChild('dataConfig')
  dataConfig : DynamicListComponent | undefined;

  @ViewChild('queryConfig')
  queryConfig : DynamicListComponent | undefined;

  @ViewChild('curlDemo')
  curlDemo: CurlDemoComponent | undefined;

  currentValue$ : Observable<IStoreWebhookConfig> | undefined;

  currentValueCorrect$: Observable<boolean> | undefined;

  urlInputValues$ : ReplaySubject<{webhookUrl: string}> =
    new ReplaySubject<{webhookUrl: string}>();

  postTypeInputValues$ : ReplaySubject<{postType: number}> =
    new ReplaySubject<{postType: number}>();

  headerInputValues$ : ReplaySubject<{ name: string,  value: string }[]> =
    new ReplaySubject<{name: string; value: string}[]>();

  dataInputValues$ : ReplaySubject<{ name: string,  value: string }[]> =
    new ReplaySubject<{name: string; value: string}[]>();

  queryInputValues$ : ReplaySubject<{ name: string,  value: string }[]> =
    new ReplaySubject<{name: string; value: string}[]>();

  currentCommand$: ReplaySubject<[{ webhookUrl: string; },
                               { postType: number; },
                               { name: string; value: string; }[],
                               { name: string; value: string; }[],
                               { name: string; value: string; }[],
                               IMyReceivedBtcChat | undefined]> =
    new ReplaySubject<[{webhookUrl: string}, {postType: number}, {name: string; value: string}[], {name: string; value: string}[], {name: string; value: string}[], (IMyReceivedBtcChat | undefined)]>();

  tooltip_url:  string = '';

  tooltip_calltype: string = '';

  tooltip_data: string = '';

  tooltip_headers: string = '';

  tooltip_query: string = '';

  tooltip_clear: string = '';

  tooltip_save: string = "";

  tooltip_save_disabled: string = '';

  constructor(private store: Store<AppState>,
              private translate: TranslateService) {

    let getStoreWebhookConfigState$: Observable<IGetStoreWebhookConfigState> =
      store.pipe(select(selectGetStoreWebhookConfigState));

    let updateStoreWebhookConfigState$: Observable<IUpdateStoreWebhookConfigState> =
      store.pipe(select(selectUpdateStoreWebhookConfigState));

    let getTestBtcChatStat$: Observable<IGetTestBtcChatState> =
      store.pipe(select(selectGetTestBtcChatState))

    this.updating$ =
      updateStoreWebhookConfigState$.pipe(select( state => state.loading));

    this.error$ =
      getStoreWebhookConfigState$.pipe(select(state => state.error));

    this.loading$ =
      getStoreWebhookConfigState$.pipe(select(state => state.loading));

    this.exampleBtcChatString$ =
      getTestBtcChatStat$.pipe(select(state => state.myReceivedBtcChat ? JSON.stringify(state.myReceivedBtcChat)
                                                                              : ""));

    this.exampleBtcChat$ =
      getTestBtcChatStat$.pipe(select(state => state.myReceivedBtcChat));

    this.translate.get('SETTINGS.CONFIGURATION_TAB_SAVE_SAVE_TOOLTIP')
      .subscribe(value => this.tooltip_save = value);

    combineLatest([this.urlInputValues$,
      this.postTypeInputValues$,
      this.headerInputValues$,
      this.dataInputValues$,
      this.queryInputValues$,
      this.exampleBtcChat$]).subscribe(
      x => {

        this.currentCommand$.next(x);
      }
    )
  }

  get hasUrlError() : boolean {
      return !!(this.urlInput?.hasError);
  }

  get hasHeaderConfigError() : boolean {
    return !!(this.headerConfig?.hasError);
  }

  get hasBodyConfigError() : boolean {
    return !!(this.dataConfig?.hasError);
  }

  get hasQueryConfigError() : boolean {
    return !!(this.queryConfig?.hasError);
  }

  get hasError() : boolean {
    return (this.hasUrlError
            || this.hasHeaderConfigError
            || this. hasBodyConfigError
            || this.hasQueryConfigError)
  }

  nextUrl(event: {webhookUrl: string}) {
    this.urlInputValues$.next(event);
  }

  nextPostType(event: {postType: number}) {
    this.postTypeInputValues$.next(event);
  }

  nextData(event: { name: string,  value: string }[]) {
    this.dataInputValues$.next(event);
  }

  nextHeaders(event: { name: string,  value: string }[]) {
    this.headerInputValues$.next(event);
  }

  nextQuery(event: { name: string,  value: string }[]) {
    this.queryInputValues$.next(event);
  }

  ngOnInit(): void {

    this.store
      .dispatch(RequestGetStoreWebhookConfigAction({payload: { }}))

    this.store
      .dispatch(RequestGetTestBtcChatAction({payload: { }}))

    this.translate.get('SETTINGS.CONFIGURATION_TAB_URL_TOOLTIP')
      .subscribe(value => this.tooltip_url = value);

    this.translate.get('SETTINGS.CONFIGURATION_TAB_CALL_TYPE_TOOLTIP')
      .subscribe(value => this.tooltip_calltype = value);

    this.translate.get('SETTINGS.CONFIGURATION_TAB_DATA_TOOLTIP')
      .subscribe(value => this.tooltip_data = value);

    this.translate.get('SETTINGS.CONFIGURATION_TAB_HEADERS_TOOLTIP')
      .subscribe(value => this.tooltip_headers = value);

    this.translate.get('SETTINGS.CONFIGURATION_TAB_QUERY_TOOLTIP')
      .subscribe(value => this.tooltip_query = value);

    this.translate.get('SETTINGS.CONFIGURATION_TAB_SAVE_TOOLTIP')
      .subscribe(value => this.tooltip_save_disabled = value);

    this.translate.get('SETTINGS.CONFIGURATION_TAB_CLEAR_TOOLTIP')
      .subscribe(value => this.tooltip_clear = value);

    this.currentValue$ =
      combineLatest([this.urlInputValues$,
                            this.postTypeInputValues$,
                            this.headerInputValues$,
                            this.dataInputValues$,
                            this.queryInputValues$])
        .pipe(map( ([urlInput,
                            postTypeInput,
                            headerInput,
                            dataInput,
                            queryInput]) =>
            this.createConfig(urlInput,
              postTypeInput,
              headerInput,
              dataInput,
              queryInput)
          )
        );

    this.currentValueCorrect$ =
      this.currentValue$
        .pipe(select(x => this.isValidWebhookConfig(x)))

  }

  createConfig(url: {webhookUrl: string},
               postType: {postType: number},
               headerValues: { name: string,  value: string }[],
               dataValues: { name: string,  value: string }[],
               queryValues: { name: string,  value: string }[]) : IStoreWebhookConfig {

    return  {
      url: url.webhookUrl,
      postTypeCode: postType.postType,
      dataAttributes: this.valuesToRecord(dataValues),
      queryAttributes: this.valuesToRecord(queryValues),
      headerAttributes: this.valuesToRecord(headerValues),
    }
  }

  valuesToRecord(values: { name: string,  value: string }[]) : Record<string, string> {
    let rvalue : Record<string, string > = {};
    for(let i = 0; i < values.length; i++) {
      let value = values[i];
      rvalue[value.name] = value.value;
    }
    return rvalue;
  }

  isValidWebhookConfig(storeWebhookConfig: IStoreWebhookConfig) : boolean {
    return !!(storeWebhookConfig.url) && !!(storeWebhookConfig.postTypeCode);
  }

  clearSettings() {
    combineLatest([this.updating$])
      .pipe(first())
      .subscribe(([updating]) =>  {

        if(updating) {
          return;
        }

        const storeWebhookConfig : IStoreWebhookConfig = {
          url: undefined,
          postTypeCode: undefined,
          dataAttributes:  undefined,
          queryAttributes:  undefined,
          headerAttributes: undefined,
        }

        const payload =
          {payload: { storeWebhookConfig: storeWebhookConfig } }

        this.store.dispatch(RequestUpdateStoreWebhookConfigAction(payload))

        // TODO set activation to false ..

      })
  }

  updateSettings() {
    if (this.hasError
        || !(this.currentValueCorrect$)
        || !(this.currentValue$)) {
      return;
    }

    combineLatest([this.updating$, this.currentValueCorrect$, this.currentValue$])
      .pipe(first())
      .subscribe(([updating, currentValueCorrect, currentValue]) =>  {

        console.info("Doing an update with: ",
                      updating,
                      currentValueCorrect,
                      currentValue)

        if(updating || !currentValueCorrect) {
          return;
        }

        const payload =
          {payload: { storeWebhookConfig: currentValue } }

        this.store.dispatch(RequestUpdateStoreWebhookConfigAction(payload))
      });
  }

}
