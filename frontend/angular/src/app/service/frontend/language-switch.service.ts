import { Injectable } from '@angular/core';
import {BehaviorSubject} from "rxjs";

@Injectable({
  providedIn: 'root'
})
export class LanguageSwitchService {

  private languageClickedEvent = new BehaviorSubject<string>('');

  emitLangueChangeEvent(msg: string){
    this.languageClickedEvent.next(msg)
  }

  langueChangeEventListner(){
    return this.languageClickedEvent.asObservable();
  }

  constructor() { }
}
