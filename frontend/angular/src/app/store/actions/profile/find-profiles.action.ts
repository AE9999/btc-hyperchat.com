import {createAction, props} from "@ngrx/store";
import {IFindProfileNamesResponse} from "../../../model/profile/find-profile-names-response";

export enum FindProfilesActionTypes {
  REQUEST_FIND_PROFILES = '[ACTION] Request Find Profiles',
  REQUEST_FIND_PROFILES_RESPONSE_OK = '[ACTION] Request Find Profiles Response OK',
  REQUEST_FIND_PROFILES_RESPONSE_ERROR = '[ACTION] Request Find Profiles Response Error',
}

export const RequestFindProfilesAction = createAction(
  FindProfilesActionTypes.REQUEST_FIND_PROFILES,
  props<{payload: { username_prefix: string} }>()
);

export const RequestFindProfilesOkAction = createAction(
  FindProfilesActionTypes.REQUEST_FIND_PROFILES_RESPONSE_OK,
  props<{payload: { findProfilesResponse: IFindProfileNamesResponse } }>()
);

export const RequestFindProfilesErrorAction = createAction(
  FindProfilesActionTypes.REQUEST_FIND_PROFILES_RESPONSE_ERROR,
  props<{payload: {  } }>()
);


