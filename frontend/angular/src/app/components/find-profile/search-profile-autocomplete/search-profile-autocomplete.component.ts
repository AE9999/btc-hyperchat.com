import { Component, OnInit } from '@angular/core';
import {FormControl} from "@angular/forms";
import {distinctUntilChanged, filter, debounceTime, Observable} from "rxjs";
import { environment } from '../../../../environments/environment';
import {ProfileService} from "../../../service/backend/profile.service";
import {select, Store} from "@ngrx/store";
import {AppState,  selectFindProfilesState} from "../../../store/reducers";
import {RequestFindProfilesAction} from "../../../store/actions/profile/find-profiles.action";
import {IFindProfilesState} from "../../../store/reducers/profile/find-profiles.reducer";
import {TranslateService} from "@ngx-translate/core";

@Component({
  selector: 'app-search-profile-autocomplete',
  templateUrl: './search-profile-autocomplete.component.html',
  styleUrls: ['./search-profile-autocomplete.component.css']
})
export class SearchProfileAutocompleteComponent implements OnInit {

  public minLengthTerm: number = environment.searchProfileAutocompleteComponent.minLengthTerm;

  public debounceTime: number = environment.searchProfileAutocompleteComponent.debounceTime;

  public error$: Observable<boolean>;

  public loading$: Observable<boolean>;

  public profileNames$: Observable<string[]>;

  control = new FormControl();

  tooltip_click: string  = '';

  constructor(private profileService: ProfileService,
              private store: Store<AppState>,
              private translate: TranslateService) {

    let searchState: Observable<IFindProfilesState>
      = store.pipe(select(selectFindProfilesState));

    this.error$ =
      searchState.pipe(select( state => state.error));

    this.loading$ =
      searchState.pipe(select( state => state.loading));

    this.profileNames$ =
      searchState.pipe(select( state => state.profileNames));

    this.translate.get('FIND_PROFILE.CLICK_TOOLTIP')
      .subscribe(value => this.tooltip_click = value);

  }

  public displayWith(value: string | undefined) : string {
    if (value) {
      return value
    }
    return "";
  }


  ngOnInit(): void {
    let me = this;
    this.control
        .valueChanges
        .pipe(
          filter(res => {
            return res && res.length >= this.minLengthTerm
          }),
          distinctUntilChanged(),
          debounceTime(me.debounceTime))
        .subscribe((value: string) => {
          const payload =  {
            payload: {
              username_prefix: value
            }
          }
          me.store.dispatch(RequestFindProfilesAction(payload))
        });
  }

  getSelectedProfile() {
    return this.control.value? this.control.value
                             : "";
  }

  inValidProfileSelected() : boolean {
    return !this.control.value ;
  }

}
