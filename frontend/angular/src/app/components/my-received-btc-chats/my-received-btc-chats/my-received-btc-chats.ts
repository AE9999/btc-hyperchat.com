import { Component, OnInit } from '@angular/core';
import {select, Store} from "@ngrx/store";
import {AppState, selectGetMyReceivedBtcChatsState } from "../../../store/reducers";
import {RequestGetMyReceivedBtcChatsAction} from "../../../store/actions/btc-chat/my-received-btc-chats/get-my-received-btc-chats.action";
import {IGetMyReceivedBtcChatsState} from "../../../store/reducers/btc-chat/my-received-btc-chats/get-my-received-btc-chats.reducer";
import {Observable} from "rxjs";
import {IMyReceivedBtcChat} from "../../../model/btc-chat/my-received-btc-chats/my-received-btc-chat";


@Component({
  selector: 'app-read-btc-chat',
  templateUrl: './my-received-btc-chats.html',
  styleUrls: ['./my-received-btc-chats.css']
})
export class MyReceivedBtcChats implements OnInit {

  loading$: Observable<boolean>;
  error$: Observable<boolean>;
  hasMyReceivedBtcChats$: Observable<boolean>;
  myReceivedBtcChats$: Observable<IMyReceivedBtcChat[]>
  getMyReceivedBtcChatsState$: Observable<IGetMyReceivedBtcChatsState>;

  constructor(private store: Store<AppState>) {
    this.getMyReceivedBtcChatsState$ =
      store.pipe(select(selectGetMyReceivedBtcChatsState));
    this.loading$ =
      this.getMyReceivedBtcChatsState$.pipe(select(state => state.loading));
    this.error$ =
      this.getMyReceivedBtcChatsState$.pipe(select(state => state.error));
    this.hasMyReceivedBtcChats$ =
      this.getMyReceivedBtcChatsState$.pipe(select(state => state.myReceivedBtcChats.length > 0))
    this.myReceivedBtcChats$ =
      this.getMyReceivedBtcChatsState$.pipe(select(state => state.myReceivedBtcChats))
  }

  ngOnInit(): void {
    this.store.dispatch(RequestGetMyReceivedBtcChatsAction({payload: { }}))
  }
}
