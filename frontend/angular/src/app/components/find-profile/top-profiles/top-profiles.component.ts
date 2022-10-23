import { Component, OnInit } from '@angular/core';
import {Observable} from "rxjs";
import {ITopProfilesState} from "../../../store/reducers/profile/top-profiles.reducer";
import {select, Store} from "@ngrx/store";
import {AppState, selectTopProfilesState} from "../../../store/reducers";
import {RequestTopProfilesAction} from "../../../store/actions/profile/top-profiles.actions";

@Component({
  selector: 'app-top-profiles',
  templateUrl: './top-profiles.component.html',
  styleUrls: ['./top-profiles.component.css']
})
export class TopProfilesComponent implements OnInit {

  topProfilesState$: Observable<ITopProfilesState>;
  topProfilesLoading$: Observable<boolean>;
  profileNames$: Observable<string[]>;
  error$: Observable<boolean>;

  constructor(private store: Store<AppState>) {
    this.topProfilesState$ = store.pipe(select(selectTopProfilesState));

    this.topProfilesLoading$ =
      this.topProfilesState$.pipe(select( state => state.loading));
    this.profileNames$ =
      this.topProfilesState$.pipe(select( state => state.profileNames));
    this.error$ =
      this.topProfilesState$.pipe(select( state => state.error));
  }

  ngOnInit(): void {
    this.store.dispatch(RequestTopProfilesAction({payload: {}}))


  }

}
