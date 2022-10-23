import { Injectable } from '@angular/core';
import {Actions, createEffect, ofType } from '@ngrx/effects';
import {MatSnackBar} from "@angular/material/snack-bar";
import {
  RequestFindProfilesAction,
  RequestFindProfilesOkAction,
  RequestFindProfilesErrorAction
} from "../../actions/profile/find-profiles.action";
import {catchError,  map, of, switchMap} from "rxjs";
import {ProfileService} from "../../../service/backend/profile.service";
import {TranslateService} from "@ngx-translate/core";

@Injectable()
export class FindProfilesEffects {
  constructor(
    private readonly actions: Actions,
    private readonly snackBar: MatSnackBar,
    private profileService: ProfileService,
    private translate: TranslateService,
  ) {}

  findProfiles$ = createEffect(() =>
    this.actions.pipe(
      ofType(RequestFindProfilesAction),
      switchMap((action)  => {
        return this.profileService.getFindProfile(action.payload.username_prefix).pipe(
          map((findProfilesResponse) =>
            RequestFindProfilesOkAction({payload: {findProfilesResponse: findProfilesResponse}})),
          catchError((error) => {

            this.translate.get('SEARCH_PROFILES_EFFECT.LOADING_ERROR')
              .subscribe(message =>
                this.snackBar.open(message, "",{duration: 2000})
              );
            return of(RequestFindProfilesErrorAction({payload: {}}))
          }),
        )
      }),
    )
  );
}
