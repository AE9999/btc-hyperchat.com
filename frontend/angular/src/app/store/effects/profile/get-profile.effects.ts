import { Injectable } from '@angular/core';
import {Actions, createEffect, ofType } from '@ngrx/effects';
import {MatSnackBar} from "@angular/material/snack-bar";
import {
  RequestGetProfileAction,
  RequestGetProfileActionResponseOkAction,
  RequestGetProfileActionResponseErrorAction
} from "../../actions/profile/get-profile.actions";
import {catchError,  map, of, switchMap} from "rxjs";
import {ProfileService} from "../../../service/backend/profile.service";
import {TranslateService} from "@ngx-translate/core";

@Injectable()
export class GetProfileEffects {
  constructor(
    private readonly actions: Actions,
    private readonly snackBar: MatSnackBar,
    private profileService: ProfileService,
    private translate: TranslateService
  ) {}

  getProfile$ = createEffect(() =>
    this.actions.pipe(
      ofType(RequestGetProfileAction),
      switchMap((action)  => {
        return this.profileService.getGetProfile(action.payload.profileName).pipe(
          map((profile) => RequestGetProfileActionResponseOkAction({payload: {requestProfileResponse: profile}})),
          catchError((error) => {

            this.translate.get('PROFILE_EFFECT.LOADING_ERROR')
              .subscribe(message =>
                this.snackBar.open(message, "",{duration: 2000})
              );
            return of(RequestGetProfileActionResponseErrorAction({payload: {}}))
          }),
        )
      }),
    )
  );
}
