import { Component, OnInit } from '@angular/core';
import {select, Store} from "@ngrx/store";
import {AppState, selectGetMyReceivedBtcChatsState, selectTopProfilesState} from "../../../store/reducers";
import {RequestTopProfilesAction} from "../../../store/actions/profile/top-profiles.actions";
import {ITopProfilesState} from "../../../store/reducers/profile/top-profiles.reducer";
import {Observable} from "rxjs";

@Component({
  selector: 'app-find-profile',
  templateUrl: './find-profile.component.html',
  styleUrls: ['./find-profile.component.css']
})
export class FindProfileComponent implements OnInit {

  constructor() {

  }

  ngOnInit(): void {
  }

}
