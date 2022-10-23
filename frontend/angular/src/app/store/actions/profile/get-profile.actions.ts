import {createAction, props} from "@ngrx/store";
import {IProfile} from "../../../model/profile/profile";


export enum GetProfileActionTypes {
  REQUEST_GET_PROFILE = '[ACTION] Request Get Profile',
  REQUEST_GET_PROFILE_RESPONSE_OK = '[ACTION] Request Get Profile Response OK',
  REQUEST_GET_PROFILE_RESPONSE_ERROR = '[ACTION] Request Get Profile Response Error',
}

export const RequestGetProfileAction = createAction(
  GetProfileActionTypes.REQUEST_GET_PROFILE,
  props<{payload: { profileName: string } }>()
);

export const RequestGetProfileActionResponseOkAction = createAction(
  GetProfileActionTypes.REQUEST_GET_PROFILE_RESPONSE_OK,
  props<{payload: { requestProfileResponse: IProfile } }>()
);

export const RequestGetProfileActionResponseErrorAction = createAction(
  GetProfileActionTypes.REQUEST_GET_PROFILE_RESPONSE_ERROR,
  props<{payload: {  } }>()
);

