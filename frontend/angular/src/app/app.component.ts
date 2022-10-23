import {Component, OnInit} from '@angular/core';
import {TranslateService} from "@ngx-translate/core";
import {LanguageSwitchService} from "./service/frontend/language-switch.service";
import {KeycloakEventType, KeycloakService} from "keycloak-angular";
import {webSocket} from "rxjs/webSocket";
import {EndpointService} from "./service/backend/endpoint.service";
import {IMyReceivedBtcChat} from "./model/btc-chat/my-received-btc-chats/my-received-btc-chat";
import {Store} from "@ngrx/store";
import {AppState} from "./store/reducers";
import {DeliverNewMyReceivedBtcChatAction} from "./store/actions/btc-chat/my-received-btc-chats/deliver-new-my-received-btc-chat.action";
import {Subject} from "rxjs";
import {environment} from "../environments/environment";
import {KeycloakEvent} from "keycloak-angular/lib/core/interfaces/keycloak-event";
import Keycloak from "keycloak-js";
import {Title} from "@angular/platform-browser";

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css']
})
export class AppComponent implements OnInit {

  title = 'frontend';

  tooltip_home: string  = '';

  tooltip_register_btc_chat: string  = '';

  tooltip_read_chats: string  = '';

  tooltip_settings: string  = '';

  tooltip_register: string  = '';

  tooltip_logout: string  = '';

  loggedin$: Subject<boolean>;

  constructor(private translate: TranslateService,
              private languageSwitchService: LanguageSwitchService,
              private endpointService: EndpointService,
              private keycloakService: KeycloakService,
              private store: Store<AppState>,
              private titleService: Title) {
    translate.addLangs(['en']);
    translate.setDefaultLang('en');
    languageSwitchService.langueChangeEventListner().subscribe(language => {
      if (language) {
        this.translate.use(language);
      }
    });

    this.translate.get('MENU.APPLICATION_TITLE')
      .subscribe(value => {
        this.title = value;
        this.titleService.setTitle(value)
      });

    this.loggedin$ = new Subject();
    this.loggedin$.next(false);

    this.translate.get('MENU.TOOLTIP_HOME')
      .subscribe(value => this.tooltip_home = value);

    this.translate.get('MENU.TOOLTIP_REGISTER_BTC_CHAT')
      .subscribe(value => this.tooltip_register_btc_chat = value);

    this.translate.get('MENU.TOOLTIP_READ_CHATS')
      .subscribe(value => this.tooltip_read_chats = value);

    this.translate.get('MENU.TOOLTIP_SETTINGS')
      .subscribe(value => this.tooltip_settings = value);

    this.translate.get('MENU.TOOLTIP_REGISTER')
      .subscribe(value => this.tooltip_register = value);

    this.translate.get('MENU.TOOLTIP_LOGOUT')
      .subscribe(value => this.tooltip_logout = value);

  }

  async ngOnInit()  {
    this.createMyReceivedBtcChatStreamWebsocket();
    this.setupLoggedinObservable();
  };

  private async setupLoggedinObservable() {

    const loggedIn = await this.keycloakService.isLoggedIn();
    this.loggedin$.next(loggedIn);
    this.keycloakService.keycloakEvents$.subscribe(keycloakEvent => {
        switch (Number(keycloakEvent)) {
          case KeycloakEventType.OnAuthError:
          case KeycloakEventType.OnAuthLogout:
          case KeycloakEventType.OnAuthRefreshError:
          case KeycloakEventType.OnTokenExpired:
            this.loggedin$.next(false);
            break;
          case KeycloakEventType.OnReady:
          case KeycloakEventType.OnActionUpdate:
            break;
          case KeycloakEventType.OnAuthRefreshSuccess:
          case KeycloakEventType.OnAuthSuccess:
            this.loggedin$.next(true);
            break;
        }
      }
    )
  }

  async register() {
    await this.keycloakService.register();
  }

  async logout() {
    await this.keycloakService.logout(environment.logoutUrl);
  }

  private async createMyReceivedBtcChatStreamWebsocket() {
    const loggedIn = await this.keycloakService.isLoggedIn();
    const me = this;
    const reconnectTime = 2000;

    if (loggedIn) {

      const subject = webSocket(this.endpointService.getMyReceivedBtcChatStreamURL());

      const token = await this.keycloakService.getToken();

      let message = `/authenticate ${token}`;

      subject.next({ message: message });

      let next =( msg : any) => {
        console.log('message received: ' + msg);
        const myReceivedBtcChat: IMyReceivedBtcChat = JSON.parse(JSON.stringify(msg));
        const payload = {
          payload:
            {
              myReceivedBtcChat: myReceivedBtcChat
            }
        };
        this.store.dispatch(DeliverNewMyReceivedBtcChatAction(payload))
      }

      let err = (err : any) => {
          console.log(err);
          setTimeout(me.createMyReceivedBtcChatStreamWebsocket.bind(me), reconnectTime)
      }

      let complete = () => {
        console.log(err);
        setTimeout(me.createMyReceivedBtcChatStreamWebsocket.bind(me), reconnectTime)
      }

      subject.subscribe( { next: next, error:  err , complete: complete} );
    } else {
      setTimeout(me.createMyReceivedBtcChatStreamWebsocket.bind(me), reconnectTime)
    }



  }


}
