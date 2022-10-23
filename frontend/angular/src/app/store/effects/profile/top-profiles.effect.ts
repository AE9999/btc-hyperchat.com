import { Injectable } from '@angular/core';
import {Actions, createEffect, ofType } from '@ngrx/effects';
import {MatSnackBar} from "@angular/material/snack-bar";
import {
  RequestTopProfilesAction,
  RequestTopProfilesResponseOkAction,
  RequestTopProfilesResponseErrorAction
} from "../../actions/profile/top-profiles.actions";
import {catchError,  map, of, switchMap} from "rxjs";
import {ProfileService} from "../../../service/backend/profile.service";
import {TranslateService} from "@ngx-translate/core";

@Injectable()
export class TopProfilesEffects {
  constructor(
    private readonly actions: Actions,
    private readonly snackBar: MatSnackBar,
    private profileService: ProfileService,
    private translate: TranslateService
  ) {}

  load$ = createEffect(() =>
    this.actions.pipe(
      ofType(RequestTopProfilesAction),
      switchMap((action)  => {
        return this.profileService.getTopProfiles().pipe(
          map((findProfilesResponse) =>
            RequestTopProfilesResponseOkAction({payload: {findProfilesResponse: findProfilesResponse}})),
          catchError((error) => {

            this.translate.get('TOP_PROFILES_EFFECT.LOADING_ERROR')
              .subscribe(message =>
                this.snackBar.open(message, "",{duration: 2000})
              );
            return of(RequestTopProfilesResponseErrorAction({payload: {}}))
          }),
        )
      }),
    )
  );
}
