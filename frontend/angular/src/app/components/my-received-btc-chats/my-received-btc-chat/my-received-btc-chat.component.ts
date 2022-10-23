import { Component, OnInit, Input } from '@angular/core';
import { IProcessBtcChatState } from "../../../store/reducers/btc-chat/my-received-btc-chats/process-btc-chat.reducer";
import {IMyReceivedBtcChat} from "../../../model/btc-chat/my-received-btc-chats/my-received-btc-chat";
import { Observable, of, map } from 'rxjs';
import {TranslateService} from "@ngx-translate/core";
import {select, Store} from "@ngrx/store";
import {AppState, selectGetMyReceivedBtcChatsState, selectProcessBtcChatState} from "../../../store/reducers";
import {RequestProcessBtcChatAction} from "../../../store/actions/btc-chat/my-received-btc-chats/process-btc-chat.action";
import {IProcessBtcChatRequest} from "../../../model/btc-chat/process-btc-chat/process-btc-chat-request";

@Component({
  selector: 'app-my-received-btc-chat',
  templateUrl: './my-received-btc-chat.component.html',
  styleUrls: ['./my-received-btc-chat.component.css']
})
export class MyReceivedBtcChatComponent implements OnInit {

  processBtcChatState$: Observable<IProcessBtcChatState>;

  @Input()
  public myReceivedBtcChat: IMyReceivedBtcChat | undefined;

  constructor(private translate: TranslateService,
              private store: Store<AppState>) {
    this.processBtcChatState$ = store.pipe(select(selectProcessBtcChatState));
  }

  ngOnInit(): void {
  }

  public sender() : Observable<string> {
    if (!this.myReceivedBtcChat
        || !this.myReceivedBtcChat.sender) {
      return this.translate.get('MY_RECEIVED_BTC_CHATS.ANONYMOUS_SENDER');
    }
    return of(this.myReceivedBtcChat.sender)
  }

  public amount(): Observable<string> {
    if (!this.myReceivedBtcChat) {
      return this.translate.get('MY_RECEIVED_BTC_CHATS.NO_AMOUNT');
    }
    let value: string =
      `${this.myReceivedBtcChat.amountInFiat} ${this.myReceivedBtcChat.currency}`;
    return of(value)
  }

  public message():Observable<string> {
    if (!this.myReceivedBtcChat
      || !this.myReceivedBtcChat.message) {
      return this.translate.get('MY_RECEIVED_BTC_CHATS.NO_MESSAGE');
    }
    return of(this.myReceivedBtcChat.message)
  }

  public receivedAt() {
    return this.myReceivedBtcChat?.dateCreated;
  }

  public isActive(): Observable<boolean> {
    if (!this.myReceivedBtcChat) {
      return of(false);
    }
    const id = this.myReceivedBtcChat.id
    return this.processBtcChatState$.pipe(
      map(processBtcChatState =>
        processBtcChatState.idsOfMyReceivedBtcChatBeingMarked.indexOf(id) >= 0)
    )
  }

  public markAsProcessed() {
    if (!this.myReceivedBtcChat) {
      return;
    }
    let payload = {
      payload: {
        processBtcChatRequest: {
           btcChatId: this.myReceivedBtcChat.id
        }
      }
    };

    this.store.dispatch(RequestProcessBtcChatAction(payload));
  }
}
