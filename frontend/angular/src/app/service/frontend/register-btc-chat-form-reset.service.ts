import { Injectable } from '@angular/core';
import {Subject} from "rxjs";

@Injectable({
  providedIn: 'root'
})
export class RegisterBtcChatFormResetService {

  public resetBtcChatForm: Subject<boolean>;

  constructor() {
    this.resetBtcChatForm = new Subject<boolean>()
  }
}
