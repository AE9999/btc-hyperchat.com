import { Injectable } from '@angular/core';
import {HttpClient} from "@angular/common/http";
import {EndpointService} from "./endpoint.service";
import {Observable} from "rxjs";
import { IProcessBtcChatRequest } from "../../model/btc-chat/process-btc-chat/process-btc-chat-request";
import { IProcessBtcChatResponse } from "../../model/btc-chat/process-btc-chat/process-btc-chat-response";
import {IMyReceivedBtcChatsResponse} from "../../model/btc-chat/my-received-btc-chats/my-received-btc-chats-response";
import {IRegisterBtcChatRequest} from "../../model/btc-chat/register-btc-chat/register-btc-chat-request";
import {IMyReceivedBtcChat} from "../../model/btc-chat/my-received-btc-chats/my-received-btc-chat";

@Injectable({
  providedIn: 'root'
})
export class BtcChatService {
  constructor(private readonly http: HttpClient,
              private readonly backend: EndpointService) { }

  public postProcessBtcChatRequest(processBtcChatRequest: IProcessBtcChatRequest): Observable<IProcessBtcChatResponse> {
    return this.http.post<IProcessBtcChatResponse>(this.backend.getApiBase() + "/btcchat/processbtcchat",
      processBtcChatRequest);
  }

  public getCurrentBtcChats(): Observable<IMyReceivedBtcChatsResponse> {
    return this.http.get<IMyReceivedBtcChatsResponse>(this.backend.getApiBase() + "/btcchat/myreceivedbtcchats");
  }

  public postRegisterBtcChatRequest(registerBtcChatRequest: IRegisterBtcChatRequest): Observable<any> {
    return this.http.post<any>(this.backend.getApiBase() + "/btcchat/registerbtcchat",
      registerBtcChatRequest);
  }

  public getTestBtcChat(): Observable<IMyReceivedBtcChat> {
    return this.http.get<IMyReceivedBtcChat>(this.backend.getApiBase() + "/btcchat/testbtcchat");
  }
}
