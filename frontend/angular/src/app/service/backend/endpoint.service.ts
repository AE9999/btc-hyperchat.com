import { Injectable } from '@angular/core';
import { environment } from '../../../environments/environment';


@Injectable({
  providedIn: 'root',
})
export class EndpointService {
  constructor() {

  }


  public getApiBase(): string {
    return environment.api;
  }

  public getMyReceivedBtcChatStreamURL(): string {
    return environment.myReceivedBtcChatStreamUrl;
  }


  public getResources(): string {
    return environment.resources;
  }
}


